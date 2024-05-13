use std::sync::{Arc, Mutex};
use std::time::Duration;

use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;
use socketioxide::extract::{AckSender, Data};
use socketioxide::{extract::SocketRef, SocketIo};
use tracing::info;

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

#[derive(Debug)]
pub struct CurrentState {
	pub value: Option<State>,
}

impl CurrentState {
	pub fn new(value: Option<State>) -> Self {
		CurrentState { value }
	}
}

lazy_static! {
	pub static ref GLOBAL_STATE: Arc<Mutex<CurrentState>> =
		Arc::new(Mutex::new(CurrentState::new(None)));
}

pub struct StateMachine {
	state_now: Option<State>,
	enter_actions: EnumMap<State, fn(&mut Self)>,
	io: SocketIo,
}

impl StateMachine {
	pub fn new(io: SocketIo) -> Self {
		let enter_actions = enum_map! {
			State::Init => StateMachine::_enter_init,
			State::Load => StateMachine::_enter_load,
			State::Running => StateMachine::_enter_running,
			State::Stopped => StateMachine::_enter_stopped,
			State::Halted => StateMachine::_enter_halted,
		};

		io.ns("/control-station", |socket: SocketRef| {
			info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
			socket.on("init", StateMachine::handle_init);
			socket.on("stop", StateMachine::handle_stop);
			socket.on("forcestop", StateMachine::handle_halt);
			socket.on("load", StateMachine::handle_load);
			socket.on("start", StateMachine::handle_run);
		});

		Self {
			state_now: None,
			enter_actions,
			io,
		}
	}

	pub async fn run(&mut self) {
		let mut interval = tokio::time::interval(TICK_INTERVAL);

		loop {
			self.tick().await;
			interval.tick().await;
		}
	}

	async fn tick(&mut self) {
		let last_state = Arc::new(Mutex::new(self.state_now));

		if self.state_now != *last_state.lock().unwrap() {
			println!(
				"State changed from {:?} to {:?}",
				*last_state.lock().unwrap(),
				Self::read_state()
			);
			self.enter_state(&self.state_now.unwrap());
		}
		if let Some(state) = Self::read_state() {
			Self::modify_state(state);
		}
		let next_state = self.state_now;
		if self.state_now == Some(State::Init) {
			Self::_init_periodic();
		}
		if self.state_now == Some(State::Running) {
			Self::_running_periodic();
		}

		*last_state.lock().unwrap() = self.state_now;

		self.pod_periodic();

		if Self::read_state().is_none() {
			self.state_now = next_state;
		} else {
			self.state_now = Self::read_state();
		}
	}

	/// Perform operations on every FSM tick
	fn pod_periodic(&mut self) {
		self.io
			.of("/control-station")
			.unwrap()
			.emit("pong", "123")
			.ok();
	}

	fn _running_periodic() {
		//println!("Rolling START state");
	}

	fn _init_periodic() {
		//println!("Rolling INIT state");
	}

	/// Run the corresponding enter action for the given state
	fn enter_state(&mut self, state: &State) {
		let enter_action = self.enter_actions[*state];
		enter_action(self);
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

	fn handle_init(Data(_data): Data<String>, ack: AckSender) {
		info!("Received init from client");
		//socket.emit("init", "init").ok();
		ack.send("init").ok();
		Self::modify_state(State::Init);
	}

	fn handle_stop(Data(_data): Data<String>, ack: AckSender) {
		info!("Received stop from client");
		//socket.emit("stop", "stop").ok();
		ack.send("stop").ok();
		Self::modify_state(State::Stopped);
	}

	fn handle_halt(Data(_data): Data<String>, ack: AckSender) {
		info!("Received halt from client");
		ack.send("halt").ok();
		Self::modify_state(State::Halted);
	}

	fn handle_load(Data(_data): Data<String>, ack: AckSender) {
		info!("Received load from client");
		//socket.emit("load", "load").ok();
		ack.send("load").ok();
		Self::modify_state(State::Load);
	}

	fn handle_run(Data(_data): Data<String>, ack: AckSender) {
		info!("Received run from client");
		//socket.emit("start", "start").ok();
		ack.send("run").ok();
		Self::modify_state(State::Running);
	}

	fn modify_state(new_value: State) {
		if let Ok(mut state) = GLOBAL_STATE.lock() {
			state.value = Some(new_value);
		}
	}

	fn read_state() -> Option<State> {
		if let Ok(state) = GLOBAL_STATE.lock() {
			state.value
		} else {
			None
		}
	}
}
