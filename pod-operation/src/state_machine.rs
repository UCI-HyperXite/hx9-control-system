use std::time::Duration;

use enum_map::{enum_map, EnumMap};
use once_cell::sync::Lazy;
use serde_json::json;
use socketioxide::extract::AckSender;
use socketioxide::{extract::SocketRef, SocketIo};
use tokio::sync::Mutex;

use tracing::info;

use crate::components::brakes::Brakes;
use crate::components::gyro::Gyroscope;
use crate::components::high_voltage_system::HighVoltageSystem;
use crate::components::lidar::Lidar;
use crate::components::lim_temperature::LimTemperature;
// use crate::components::pico_relay::PicoRelay;
use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;
use crate::components::wheel_encoder::WheelEncoder;

const TICK_INTERVAL: Duration = Duration::from_millis(100);
const STOP_THRESHOLD: f32 = 37.0; // Meters
const MIN_PRESSURE: f32 = 126.0; // PSI
const END_OF_TRACK: f32 = 8.7; // Meters
const LIM_TEMP_THRESHOLD: f32 = 71.0; //°C
const BRAKING_THRESHOLD: f32 = 9.1; // Meters
const BRAKING_DECELERATION: f32 = -15.14; // m/s^2
const ENCODER_SAMPLE_INTERVAL: Duration = Duration::from_micros(100);

#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_map::Enum)]
pub enum State {
	Init,
	Load,
	Running,
	Stopped,
	Halted,
	Faulted,
}

type StateTransition = fn(&mut StateMachine) -> State;

pub struct StateMachine {
	last_state: State,
	state: &'static Mutex<State>,
	enter_actions: EnumMap<State, fn(&mut Self)>,
	state_transitions: EnumMap<State, Option<StateTransition>>,
	io: SocketIo,
	brakes: Brakes,
	signal_light: SignalLight,
	upstream_pressure_transducer: PressureTransducer,
	downstream_pressure_transducer: PressureTransducer,
	lim_temperature_port: LimTemperature,
	lim_temperature_starboard: LimTemperature,
	high_voltage_system: HighVoltageSystem,
	lidar: Lidar,
	wheel_encoder: std::sync::Arc<std::sync::Mutex<WheelEncoder>>,
	gyro: Gyroscope,
	// pico_relay: PicoRelay,
}

impl StateMachine {
	pub fn new(io: SocketIo) -> Self {
		static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::Init));

		let enter_actions = enum_map! {
			State::Init => StateMachine::_enter_init,
			State::Load => StateMachine::_enter_load,
			State::Running => StateMachine::_enter_running,
			State::Stopped => StateMachine::_enter_stopped,
			State::Halted => StateMachine::_enter_halted,
			State::Faulted => StateMachine::_enter_faulted,
		};

		let state_transitions = enum_map! {
			State::Init => None,
			State::Load => Some(StateMachine::_load_periodic as fn(&mut Self) -> State),
			State::Running => Some(StateMachine::_running_periodic as fn(&mut Self) -> State),
			State::Stopped => None,
			State::Halted => None,
			State::Faulted => None,
		};

		io.ns("/control-station", |socket: SocketRef| {
			info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
			socket.on("load", |ack: AckSender| async {
				Self::handle_load(&STATE, ack).await;
			});
			socket.on("run", |ack: AckSender| async {
				Self::handle_run(&STATE, ack).await;
			});
			socket.on("stop", |ack: AckSender| async {
				Self::handle_stop(&STATE, ack).await;
			});
			socket.on("halt", |ack: AckSender| async {
				Self::handle_halt(&STATE, ack).await;
			});
		});

		Self {
			last_state: State::Init,
			state: &STATE,
			enter_actions,
			state_transitions,
			io,
			brakes: Brakes::new(),
			signal_light: SignalLight::new(),
			upstream_pressure_transducer: PressureTransducer::upstream(),
			downstream_pressure_transducer: PressureTransducer::downstream(),
			lim_temperature_port: LimTemperature::new(ads1x1x::SlaveAddr::Default),
			lim_temperature_starboard: LimTemperature::new(ads1x1x::SlaveAddr::Alternative(
				false, true,
			)),
			high_voltage_system: HighVoltageSystem::new(),
			lidar: Lidar::new(),
			gyro: Gyroscope::new(),
			wheel_encoder: std::sync::Arc::new(std::sync::Mutex::new(WheelEncoder::new())),
			// pico_relay: PicoRelay::new(),
		}
	}

	pub async fn run(&mut self) {
		let mut interval = tokio::time::interval(TICK_INTERVAL);
		let encoder = self.wheel_encoder.clone();

		tokio::spawn(Self::wheel_encoder_task(encoder));

		loop {
			self.tick().await;
			interval.tick().await;
		}
	}

	async fn wheel_encoder_task(wheel_encoder: std::sync::Arc<std::sync::Mutex<WheelEncoder>>) {
		let mut interval = tokio::time::interval(ENCODER_SAMPLE_INTERVAL);

		loop {
			// Lock the mutex, measure, then immediately unlock
			let _value = {
				let mut encoder = wheel_encoder.lock().unwrap();
				match encoder.measure() {
					Ok(value) => value,
					Err(e) => {
						info!("Wheel encoder error: {:?}", e);
						continue;
					}
				}
			};
			//info!("Wheel encoder value: {}", value);
			interval.tick().await;
		}
	}

	/// Tick the state machine by running the transition for the current state
	/// and actions for when entering a new state
	async fn tick(&mut self) {
		// Acquire lock for state to prevent socket handlers from overwriting
		let mut state = self.state.lock().await;

		// Run enter action when entering a new state
		if *state != self.last_state {
			info!("State changed from {:?} to {:?}", self.last_state, state);
			self.enter_state(&state);
		}

		self.pod_periodic();

		// Proceed to the next state by transition
		let next_state = self.run_state_transition(&state);
		self.last_state = *state;
		*state = next_state;
		// state is dropped, releasing the lock
	}

	/// Perform operations on every FSM tick
	fn pod_periodic(&mut self) {
		// Reading each value individually
		let gyro_data = self.gyro.read_orientation();
		let downstream_pressure_data = self.downstream_pressure_transducer.read_pressure();
		let upstream_pressure_data = self.upstream_pressure_transducer.read_pressure();
		let lim_temp_port_data = self.lim_temperature_port.read_lim_temps();
		let lim_temp_starboard_data = self.lim_temperature_starboard.read_lim_temps();

		// Full JSON object
		let full_json = json!({
			"gyroscope": gyro_data,
			"downstream_pressure_transducer": downstream_pressure_data,
			"upstream_pressure_transducer": upstream_pressure_data,
			"lim_temperature_port": lim_temp_port_data,
			"lim_temperature_starboard": lim_temp_starboard_data,
		});

		self.io
			.of("/control-station")
			.unwrap()
			.emit("serverResponse", full_json)
			.ok();
	}

	/// Run the corresponding enter action for the given state
	fn enter_state(&mut self, state: &State) {
		let enter_action = self.enter_actions[*state];
		enter_action(self);
	}

	/// Run the transition function for a given state if it exists.
	/// Otherwise, remain in the same state.
	fn run_state_transition(&mut self, state: &State) -> State {
		match self.state_transitions[*state] {
			Some(state_transition) => state_transition(self),
			None => *state,
		}
	}

	fn _enter_init(&mut self) {
		info!("Entering Init state");
		self.signal_light.disable();
	}

	fn _enter_load(&mut self) {
		info!("Entering Load state");
		self.brakes.disengage();
		self.signal_light.disable();
		// self.pico_relay.disable();
	}

	fn _enter_running(&mut self) {
		info!("Entering Running state");
		self.high_voltage_system.enable(); // Enable high voltage system -- may move later
		self.signal_light.enable();
		self.brakes.disengage();
		// self.pico_relay.enable();
	}

	fn _enter_stopped(&mut self) {
		info!("Entering Stopped state");
		self.signal_light.disable();
		self.brakes.engage();
		// self.pico_relay.disable();
	}

	fn _enter_halted(&mut self) {
		info!("Entering Halted state");
		self.signal_light.disable();
		self.brakes.engage();
		self.high_voltage_system.disable();
		// self.pico_relay.disable();
	}

	fn _enter_faulted(&mut self) {
		info!("Entering Faulted state");
		self.signal_light.disable();
		self.brakes.engage();
		self.high_voltage_system.disable();
		// self.pico_relay.disable();
	}

	/// Perform operations when the pod is loading
	fn _load_periodic(&mut self) -> State {
		info!("Rolling Load state");
		State::Load
	}

	/// Perform operations when the pod is running
	fn _running_periodic(&mut self) -> State {
		info!("Rolling Running state");

		let encoder_value = self.wheel_encoder.lock().unwrap();
		let distance = encoder_value.get_distance();
		let velocity = encoder_value.get_velocity();
		drop(encoder_value);

		let full_json = json!({
			"wheel_encoder": {
				"distance": distance,
				"velocity": velocity,
			},
		});

		self.io
			.of("/control-station")
			.unwrap()
			.emit("serverResponse", full_json)
			.ok();

		if StateMachine::_should_stop(distance, velocity) {
			return State::Stopped;
		}

		if self.downstream_pressure_transducer.read_pressure() < MIN_PRESSURE {
			self.io
				.of("/control-station")
				.unwrap()
				.emit(
					"fault",
					(
						"Low pressure detected. Currently {}, should be above 126 PSI.",
						self.downstream_pressure_transducer.read_pressure(),
					),
				)
				.ok();
			return State::Faulted;
		}

		let default_readings = self.lim_temperature_port.read_lim_temps();
		let alternative_readings = self.lim_temperature_starboard.read_lim_temps();
		if default_readings
			.iter()
			.chain(alternative_readings.iter())
			.any(|&reading| reading > LIM_TEMP_THRESHOLD)
		{
			self.io
				.of("/control-station")
				.unwrap()
				.emit(
					"fault",
					(
						"High temperature detected, should be below {} C.",
						LIM_TEMP_THRESHOLD,
					),
				)
				.ok();
			return State::Faulted;
		}
		// Last 20% of the track, as indicated by braking
		if self.lidar.read_distance() < END_OF_TRACK {
			self.io
				.of("/control-station")
				.unwrap()
				.emit("fault", ("End of track detected. Current distance to end: {}, less than {} meters away", self.lidar.read_distance(), END_OF_TRACK))
				.ok();
			return State::Faulted;
		}

		State::Running
	}

	/// Consider two stopping conditions based on the pod's distance and velocity
	/// at the next tickwhich is when the stopping will actually initiate
	fn _should_stop(distance: f32, velocity: f32) -> bool {
		// Predict next tick's braking distance
		let predicted_velocity = velocity + BRAKING_DECELERATION * TICK_INTERVAL.as_secs_f32();
		let predicted_braking_distance = -predicted_velocity.powi(2) / (2.0 * BRAKING_DECELERATION);

		// Check if the predicted braking distance requires stopping
		if distance + velocity * TICK_INTERVAL.as_secs_f32() >= STOP_THRESHOLD {
			return true;
		}

		predicted_braking_distance <= BRAKING_THRESHOLD
	}

	// To avoid conflicts with the state-transition model,
	// each of these event handlers must wait for an ongoing transition to complete
	// by awaiting the mutex lock to be acquired.
	// Tokio::sync::mutex uses a fairness queue to ensure in-order acquisition.
	// The acknowledgement is then sent after the state is updated.

	async fn handle_load(state: &Mutex<State>, ack: AckSender) {
		info!("Received load from client");
		let mut state = state.lock().await;
		*state = State::Load;
		ack.send("load").ok();
	}

	async fn handle_run(state: &Mutex<State>, ack: AckSender) {
		info!("Received start from client");

		let mut state = state.lock().await;
		*state = State::Running;
		ack.send("run").ok();
	}

	async fn handle_stop(state: &Mutex<State>, ack: AckSender) {
		info!("Received stop from client");
		let mut state = state.lock().await;
		*state = State::Stopped;
		ack.send("stop").ok();
	}

	async fn handle_halt(state: &Mutex<State>, ack: AckSender) {
		info!("Received halt from client");
		let mut state = state.lock().await;
		*state = State::Halted;
		ack.send("halt").ok();
	}
}
