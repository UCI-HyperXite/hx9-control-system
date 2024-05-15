use axum::Server;
use socketioxide::SocketIo;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod components;
mod demo;
mod state_machine;

use crate::components::lim_temperature::LimTemperature;
use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;
use crate::components::wheel_encoder::WheelEncoder;
use crate::state_machine::StateMachine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing::subscriber::set_global_default(FmtSubscriber::default())?;

	let (layer, io) = SocketIo::new_layer();

	let signal_light = SignalLight::new();
	tokio::spawn(demo::blink(signal_light));

	let pressure_transducer = PressureTransducer::new(0x40);
	tokio::spawn(demo::read_pressure_transducer(pressure_transducer));

	let ads1015 = LimTemperature::new(ads1x1x::SlaveAddr::Default);
	tokio::spawn(demo::read_ads1015(ads1015));

	let wheel_encoder = WheelEncoder::new();
	tokio::spawn(demo::read_wheel_encoder(wheel_encoder));
	wheel_encoder.reset();

	tokio::spawn(async {
		let mut state_machine = StateMachine::new(io);
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
