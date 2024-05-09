use axum::Server;
use socketioxide::{extract::SocketRef, SocketIo};
use socketioxide::extract::{AckSender, Data};

use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
mod state;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use state::{State, GLOBAL_STATE};
use std::collections::HashMap;

struct StateMachine {
	state_now: Option<State>,
    enter_actions: HashMap<State, fn()>, 
	io: SocketIo,
}

impl StateMachine {
	fn new(io: SocketIo) -> Self {
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
			io
		}
	}

	fn run(&mut self) {
		let last_state = Arc::new(Mutex::new(self.state_now));
		loop {

			if self.state_now.clone() != *last_state.lock().unwrap() {
				println!("State changed from {:?} to {:?}", *last_state.lock().unwrap(), Self::read_state());
					self.enter_state(&self.state_now.clone().unwrap());
			}	
			if let Some(state) = Self::read_state() {
				Self::modify_state(state);
			}
			let next_state = self.state_now.clone();
			if self.state_now == Some(State::Init){
				Self::_init_periodic();
			}
			if self.state_now == Some(State::Start){
				Self::_running_periodic();
			}

			*last_state.lock().unwrap() = self.state_now;

			self.sensor_data();

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

	pub fn handle_init(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
		info!("Received init from client");
		socket.emit("init", "init").ok();
		Self::modify_state(State::Init);
	}

	pub fn handle_stop(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
		info!("Received stop from client");
		socket.emit("stop", "stop").ok();
		Self::modify_state(State::Stop);
	}

	fn handle_forcestop(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
		info!("Received forcestop from client");
		socket.emit("forcestop", "forcestop").ok();
		Self::modify_state(State::ForceStop);
	}

	fn handle_load(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
		info!("Received load from client");
		socket.emit("load", "load").ok();
		Self::modify_state(State::Load);
	}

	fn handle_start(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
		info!("Received start from client");
		socket.emit("start", "start").ok();
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
		self.io.emit("sensor_data", [1, 2, 3, 4]).ok();
		thread::sleep(Duration::from_secs(1));
		
	}
				
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {	

	tracing::subscriber::set_global_default(FmtSubscriber::default())?;

	let (layer, io) = SocketIo::new_layer();

	thread::sleep(Duration::from_millis(1));

	thread::spawn(move || {
		let mut state_machine = StateMachine::new(io);
		state_machine.run();
	});

	let app = axum::Router::new().layer(layer);

	info!("Starting server on port 5000");
	
	let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}

	Ok(())
}
