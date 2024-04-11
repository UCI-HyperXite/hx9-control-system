use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Init,
    Load,
    Running,
    Stopped,
    ForceStop
}

impl State {
    pub fn transition(self, event: &str) -> Self {
        match (self, event) {
            (State::Init, "start") => State::Running,
            (State::Running, "stop") => State::Stopped,
            (State::Stopped, "service") => State::Service,
            (State::load, "start") => State::Running,
            _ => self,
        }
    }
}

pub type SharedState = Arc<Mutex<State>>;