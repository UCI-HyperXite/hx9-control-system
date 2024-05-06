use axum::Server;
use socketioxide::{extract::SocketRef, SocketIo};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
mod state;
mod components;
// mod demo;
mod handlers;
// use crate::components::pressure_transducer::PressureTransducer;
// use crate::components::signal_light::SignalLight;

use std::{sync::{Arc, Mutex}, thread, time::Duration};
use state::State;
use handlers::{modify_state, read_state};
use std::collections::HashMap;

struct EnterActions {
    enter_actions: HashMap<State, fn()>, 
}

impl EnterActions {
    fn new() -> Self {
        let mut enter_actions = HashMap::new();
        let mut enter_actions_map = HashMap::new();
        enter_actions_map.insert(State::Init, EnterActions::_enter_init as fn());
		enter_actions_map.insert(State::Start, EnterActions::_enter_start as fn());
		enter_actions_map.insert(State::Stop, EnterActions::_enter_stop as fn());
		enter_actions_map.insert(State::ForceStop, EnterActions::_enter_forcestop as fn());
		enter_actions_map.insert(State::Load, EnterActions::_enter_load as fn());
        enter_actions.extend(enter_actions_map);
		EnterActions {
            enter_actions,
        }
    }

	fn _enter_init() {
        println!("Entering INIT state");
    }

    fn _enter_load() {
        println!("Entering LOAD state");
		modify_state(State::Init);
    }

    fn _enter_start() {
        println!("Entering START state");
    }

    fn _enter_stop() {
        println!("Entering STOP state");
		modify_state(State::Init);
		println!("State changed to {:?}", read_state());
    }

	fn _enter_forcestop() {
        println!("Entering FORCESTOP state");
		modify_state(State::Init);
    }

	fn enter_state(&self, state: &State) {
		println!("Entering state: {:?}", state);
		if let Some(enter_action) = self.enter_actions.get(state) {
			enter_action();
		} else {
			println!("No enter action defined for {:?}", state);
		}
	}
}
struct StateTransitions {
    _state_transitions: HashMap<State, fn()>,
}

impl StateTransitions {
	fn new() -> Self {
		let mut _state_transitions = HashMap::new();
		_state_transitions.insert(State::Start, Self::_running_periodic as fn());
		_state_transitions.insert(State::Init, Self::_init_periodic as fn());
		Self {
			_state_transitions,
		}
	}

	fn _running_periodic() {
		//println!("Rolling START state");
	}
	 
	fn _init_periodic() {
		//println!("Rolling INIT state");
	}
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut state_now: Option<State> = Some(State::Init);
	let last_state = Arc::new(Mutex::new(state_now));
	let enter_actions = EnterActions::new();
	
	thread::spawn(move || {
		loop {
			let value = read_state();
			   //println!("State value: {:?}", value);
			   //println!("Last state value: {:?}", *last_state.lock().unwrap());

			if state_now.clone() != *last_state.lock().unwrap() {
				println!("State changed from {:?} to {:?}", *last_state.lock().unwrap(), read_state());
					enter_actions.enter_state(&state_now.clone().unwrap());
			}	
			//add send data function here 
			if let Some(state) = read_state() {
				modify_state(state);
			}
			let next_state = state_now.clone();
			if state_now == Some(State::Init){
				StateTransitions::_init_periodic();
			}
			if state_now == Some(State::Start){
				StateTransitions::_running_periodic();
			}

			*last_state.lock().unwrap() = state_now;


			if read_state() == None {
				state_now = next_state;
			} else {
				state_now = read_state();
			}


		}
		});

	tracing::subscriber::set_global_default(FmtSubscriber::default())?;
		let (layer, io) = SocketIo::new_layer();
		thread::sleep(Duration::from_millis(1));
		io.ns("/control-station", |socket: SocketRef| {
			info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
			socket.on("init", handlers::handle_init);
			socket.on("stop", handlers::handle_stop);
			socket.on("forcestop", handlers::handle_forcestop);
			socket.on("load", handlers::handle_load);
			socket.on("start", handlers::handle_start);

		});

	// let signal_light = SignalLight::new();
	// tokio::spawn(demo::blink(signal_light));

	// let pressure_transducer = PressureTransducer::new(0x40);
	// tokio::spawn(demo::read_pressure_transducer(pressure_transducer));

		let app = axum::Router::new().layer(layer);
	

	info!("Starting server on port 5000");
	
	let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}
	

	
	Ok(())
}
