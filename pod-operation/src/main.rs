use axum::Server;
use socketioxide::SocketIo;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod components;
mod demo;
mod state_machine;

use crate::components::brakes::Brakes;
use crate::components::lim_current::LimCurrent;
use crate::components::gyro::Gyroscope;
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

	let upstream_pressure_transducer = PressureTransducer::upstream();
	tokio::spawn(demo::read_pressure_transducer(upstream_pressure_transducer));

	let downstream_pressure_transducer = PressureTransducer::downstream();
	tokio::spawn(demo::read_pressure_transducer(
		downstream_pressure_transducer,
	));

	let ads1015 = LimTemperature::new(ads1x1x::SlaveAddr::Default);
	tokio::spawn(demo::read_ads1015(ads1015));

	let wheel_encoder = WheelEncoder::new();
	tokio::spawn(demo::read_wheel_encoder(wheel_encoder));

	let gyro: Gyroscope = Gyroscope::new();
	tokio::spawn(demo::read_gyroscope(gyro));
	let brakes = Brakes::new();
	tokio::spawn(demo::brake(brakes));

	tokio::spawn(async {
		let mut state_machine = StateMachine::new(io);
		state_machine.run().await;
	});

	let limcurrent = LimCurrent::new(ads1x1x::SlaveAddr::Default);
	tokio::spawn(demo::read_lim_current(limcurrent));

	let app = axum::Router::new().layer(layer);

	info!("Starting server on port 5000");

	let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}

	Ok(())
}
