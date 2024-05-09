use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

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
