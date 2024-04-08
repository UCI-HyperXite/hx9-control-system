use axum::Server;
use socketioxide::{extract::SocketRef, SocketIo};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod components;
mod demo;
mod handlers;

use crate::components::ina219::read_current;
use crate::components::signal_light::SignalLight;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing::subscriber::set_global_default(FmtSubscriber::default())?;

	let (layer, io) = SocketIo::new_layer();

	io.ns("/control-station", |socket: SocketRef| {
		info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
		socket.on("ping", handlers::handle_ping);
	});

	let signal_light = SignalLight::new();
	tokio::spawn(demo::blink(signal_light));

	tokio::spawn(read_current());

	let app = axum::Router::new().layer(layer);

	info!("Starting server on port 5000");
	let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}

	Ok(())
}
