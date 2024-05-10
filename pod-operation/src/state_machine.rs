use socketioxide::extract::{AckSender, Data};
use socketioxide::{extract::SocketRef, SocketIo};

use crate::components::signal_light::SignalLight;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
	Start,
	Stop,
	ForceStop,
	Load,
	Init,
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
	enter_actions: HashMap<State, fn()>,
	io: SocketIo,
}

impl StateMachine {
	pub fn new(io: SocketIo) -> Self {
		let mut enter_actions = HashMap::new();
		enter_actions.insert(State::Init, StateMachine::_enter_init as fn());
		enter_actions.insert(State::Start, StateMachine::_enter_start as fn());
		enter_actions.insert(State::Stop, StateMachine::_enter_stop as fn());
		enter_actions.insert(State::ForceStop, StateMachine::_enter_forcestop as fn());
		enter_actions.insert(State::Load, StateMachine::_enter_load as fn());
		io.ns("/", |socket: SocketRef| {
			info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
			socket.on("init", StateMachine::handle_init);
			socket.on("stop", StateMachine::handle_stop);
			socket.on("forcestop", StateMachine::handle_forcestop);
			socket.on("load", StateMachine::handle_load);
			socket.on("start", StateMachine::handle_start);
		});
		Self {
			state_now: None,
			enter_actions,
			io,
		}
	}

	pub fn run(&mut self) {
		let last_state = Arc::new(Mutex::new(self.state_now));
		let mut signal_light = SignalLight::new();

		loop {
			if self.state_now.clone() != *last_state.lock().unwrap() {
				println!(
					"State changed from {:?} to {:?}",
					*last_state.lock().unwrap(),
					Self::read_state()
				);
				self.enter_state(&self.state_now.clone().unwrap());
			}
			if let Some(state) = Self::read_state() {
				Self::modify_state(state);
			}
			let next_state = self.state_now.clone();
			if self.state_now == Some(State::Init) {
				signal_light.disable();
				Self::_init_periodic();
			}
			if self.state_now == Some(State::Start) {
				signal_light.enable();
				Self::_running_periodic();
			}

			*last_state.lock().unwrap() = self.state_now;

			//self.sensor_data();

			if Self::read_state() == None {
				self.state_now = next_state;
			} else {
				self.state_now = Self::read_state();
			}
		}
	}

	fn _running_periodic() {
		//println!("Rolling START state");
	}

	fn _init_periodic() {
		//println!("Rolling INIT state");
	}

	fn _enter_init() {
		println!("Entering INIT state");
	}

	fn _enter_load() {
		println!("Entering LOAD state");
		Self::modify_state(State::Init);
	}

	fn _enter_start() {
		println!("Entering START state");
	}

	fn _enter_stop() {
		println!("Entering STOP state");
		Self::modify_state(State::Init);
		println!("State changed to {:?}", Self::read_state());
	}

	fn _enter_forcestop() {
		println!("Entering FORCESTOP state");
		Self::modify_state(State::Init);
	}

	fn enter_state(&self, state: &State) {
		println!("Entering state: {:?}", state);
		if let Some(enter_action) = self.enter_actions.get(state) {
			enter_action();
		} else {
			println!("No enter action defined for {:?}", state);
		}
	}

	fn handle_init(socket: SocketRef, Data(_data): Data<String>, ack: AckSender) {
		info!("Received init from client");
		//socket.emit("init", "init").ok();
		ack.send("init").ok();
		Self::modify_state(State::Init);
	}

	fn handle_stop(socket: SocketRef, Data(_data): Data<String>, ack: AckSender) {
		info!("Received stop from client");
		//socket.emit("stop", "stop").ok();
		ack.send("stop").ok();
		Self::modify_state(State::Stop);
	}

	fn handle_forcestop(socket: SocketRef, Data(_data): Data<String>, ack: AckSender) {
		info!("Received forcestop from client");
		//socket.emit("forcestop", "forcestop").ok();
		ack.send("forcestop").ok();
		Self::modify_state(State::ForceStop);
	}

	fn handle_load(socket: SocketRef, Data(_data): Data<String>, ack: AckSender) {
		info!("Received load from client");
		//socket.emit("load", "load").ok();
		ack.send("load").ok();
		Self::modify_state(State::Load);
	}

	fn handle_start(socket: SocketRef, Data(_data): Data<String>, ack: AckSender) {
		info!("Received start from client");
		//socket.emit("start", "start").ok();
		ack.send("start").ok();
		Self::modify_state(State::Start);
	}

	fn modify_state(new_value: State) {
		if let Ok(mut state) = GLOBAL_STATE.lock() {
			state.value = Some(new_value);
		}
	}

	fn read_state() -> Option<State> {
		if let Ok(state) = GLOBAL_STATE.lock() {
			state.value.clone()
		} else {
			None
		}
	}

	fn sensor_data(&self) {
		self.io.emit("sensor_data", [1, 2, 3]).ok();
	}
}
