use axum::Server;
use socketioxide::SocketIo;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
mod components;
mod demo;
mod state_machine;
use crate::state_machine::StateMachine;
use std::{thread, time::Duration};

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
