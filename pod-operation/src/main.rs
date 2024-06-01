use axum::Server;
use socketioxide::SocketIo;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod components;
mod demo;
mod state_machine;
mod utils;

use crate::state_machine::StateMachine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing::subscriber::set_global_default(FmtSubscriber::default())?;

	#[cfg(not(feature = "rpi"))]
	info!("NOTE: Did not compile for Raspberry Pi, peripherals will be mocked.");

	let (layer, io) = SocketIo::new_layer();

	let mut state_machine = StateMachine::new(io);
	tokio::spawn(async move {
		state_machine.run().await;
	});

	let app = axum::Router::new().layer(layer);

	info!("Starting server on port 5000");

	let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}

	Ok(())
}
