use std::time::Duration;

use enum_map::{enum_map, EnumMap};
use once_cell::sync::Lazy;
use socketioxide::extract::AckSender;
use socketioxide::{extract::SocketRef, SocketIo};
use tokio::sync::Mutex;
use tracing::info;
use crate::components::pressure_transducer::PressureTransducer;

// use crate::components::signal_light::SignalLight;

const TICK_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_map::Enum)]
pub enum State {
	Init,
	Load,
	Running,
	Stopped,
	Halted,
}

type StateTransition = fn(&mut StateMachine) -> State;

pub struct StateMachine {
	last_state: State,
	state: &'static Mutex<State>,
	enter_actions: EnumMap<State, fn(&mut Self)>,
	state_transitions: EnumMap<State, Option<StateTransition>>,
	io: SocketIo,
	upstream_pressure_transducer: PressureTransducer,
	downstream_pressure_transducer: PressureTransducer,
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
		};

		let state_transitions = enum_map! {
			State::Init => None,
			State::Load => Some(StateMachine::_load_periodic as fn(&mut Self) -> State),
			State::Running => Some(StateMachine::_running_periodic as fn(&mut Self) -> State),
			State::Stopped => None,
			State::Halted => None,
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
			upstream_pressure_transducer: PressureTransducer::upstream(),
			downstream_pressure_transducer: PressureTransducer::downstream(),
		}
	}

	pub async fn run(&mut self) {
		let mut interval = tokio::time::interval(TICK_INTERVAL);

		loop {
			self.tick().await;
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
		self.io
			.of("/control-station")
			.unwrap()
			.emit("pong", "123")
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
	}

	fn _enter_load(&mut self) {
		info!("Entering Load state");
	}

	fn _enter_running(&mut self) {
		info!("Entering Running state");
		// self.signal_light.enable();
	}

	fn _enter_stopped(&mut self) {
		info!("Entering Stopped state");
		// self.signal_light.disable();
	}

	fn _enter_halted(&mut self) {
		info!("Entering Halted state");
		// self.hvs.disable()
	}

	/// Perform operations when the pod is loading
	fn _load_periodic(&mut self) -> State {
		info!("Rolling Load state");

		State::Load
	}

	/// Perform operations when the pod is running
	fn _running_periodic(&mut self) -> State {
		info!("Rolling Running state");
		if self.upstream_pressure_transducer.read_pressure() > 1000.0 {
			State::Ha
		} else {
			State::Running
		}
		State::Running
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
