use axum::Server;
use socketioxide::{extract::SocketRef, SocketIo};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

use axum::Server;
use socketioxide::SocketIo;
use statig::{state_machine, Response::{Super, Transition}};
use tracing::{error, info};
use axum::CorsLayer;
use tracing_subscriber::FmtSubscriber;
use statig::Response;
use socketioxide::extract::{AckSender, Data, SocketRef};

mod handlers;

#[derive(Default)]
pub struct StateMachine {
    socket: SocketRef,
}
pub enum Event {
    Init,
    ClientLoad,
    ClientRunning,
    ClientStop,
}

#[state_machine(initial = "State::init()")]
impl StateMachine {
    #[state]
    fn load(&mut self, event: &Event) -> Response<State> {
        // Emit event to client
        self.socket.emit("load_event", "Load event occurred").ok();
        
        // State transitions
        match event {
            Event::ClientLoad => Transition(State::load()),
            Event::ClientRunning => Transition(State::running()),
            _ => Super,
        }
    }

    #[state]
    fn running(&mut self, event: &Event) -> Response<State> {
        // Emit event to client
        self.socket.emit("running_event", "Running event occurred").ok();

        // State transitions
        match event {
            Event::ClientStop => Transition(State::stop()),
            _ => Super,
        }
    }

    #[state]
    fn init(&mut self, event: &Event) -> Response<State> {
        // Emit event to client
        self.socket.emit("init_event", "Init event occurred").ok();

        // State transitions
        match event {
            Event::ClientRunning => Transition(State::running()),
            Event::ClientLoad => Transition(State::stop()),
            _ => Super,
        }
    }

    #[state]
    fn stop(&mut self) -> Response<State> {
        // Emit event to client
        self.socket.emit("stop_event", "Stop event occurred").ok();

        // State transition
        Transition(State::init())
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;


    let (layer, io) = SocketIo::new_layer();

    io.ns("/control-station", |socket: SocketRef| {
        info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
        socket.on("ping", handle_ping);
    });

    let app = axum::Router::new().layer(layer).layer(CorsLayer::new());

    info!("Starting server on port 5000");
    let server = Server::bind(&"0.0.0.0:5000".parse().unwrap()).serve(app.into_make_service());

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}
