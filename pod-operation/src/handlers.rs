use socketioxide::extract::{AckSender, Data, SocketRef};
use tracing::info;
use crate::state::{GLOBAL_STATE, State};

pub fn modify_state(new_value: State) {
    if let Ok(mut state) = GLOBAL_STATE.lock() {
        state.value = Some(new_value);
    }
}

pub fn read_state() -> Option<State> {
    if let Ok(state) = GLOBAL_STATE.lock() {
        state.value.clone()
    } else {
        None 
    }
}

pub fn handle_init(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
	info!("Received init from client");
	socket.emit("init", "init").ok();
	ack.send("init").ok();
	modify_state(State::Init);
}

pub fn handle_stop(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
	info!("Received stop from client");
	socket.emit("stop", "stop").ok();
	ack.send("stop").ok();
	modify_state(State::Stop);
}

pub fn handle_forcestop(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
	info!("Received forcestop from client");
    socket.emit("forcestop", "forcestop").ok();
    ack.send("forcestop").ok();
	modify_state(State::ForceStop);
}

pub fn handle_load(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
	info!("Received 3 from client");
    socket.emit("load", "load").ok();
    ack.send("load").ok();
	modify_state(State::Load);
}

pub fn handle_start(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
	info!("Received 3 from client");
    socket.emit("start", "start").ok();
    ack.send("start").ok();
	modify_state(State::Start);
}