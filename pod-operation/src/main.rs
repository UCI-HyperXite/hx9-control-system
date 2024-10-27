use axum::{routing::Router, Server};
use socketioxide::SocketIo;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod components;
mod demo;
mod state_machine;
mod utils;

use crate::components::brakes::Brakes;
use crate::components::gyro::Gyroscope;
use crate::components::high_voltage_system::HighVoltageSystem;
// use crate::components::inverter_board::InverterBoard;
// use crate::components::lidar::Lidar;
use crate::components::lim_current::LimCurrent;
use crate::components::lim_temperature::LimTemperature;
// use crate::components::motors::Motors;
// use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;
use crate::components::wheel_encoder::WheelEncoder;
// use crate::state_machine::StateMachine;
// use components::pico_relay::PicoRelay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing::subscriber::set_global_default(FmtSubscriber::default())?;

	#[cfg(not(feature = "rpi"))]
	info!("NOTE: Did not compile for Raspberry Pi, peripherals will be mocked.");

	let (layer, io) = SocketIo::new_layer();

	let signal_light = SignalLight::new();
	tokio::spawn(demo::blink(signal_light));

	// let pico_relay = PicoRelay::new();
	// tokio::spawn(demo::blink_relay(pico_relay));

	// let upstream_pressure_transducer = PressureTransducer::upstream();
	// tokio::spawn(demo::read_pressure_transducer(upstream_pressure_transducer));

	// let downstream_pressure_transducer = PressureTransducer::downstream();
	// tokio::spawn(demo::read_pressure_transducer(
	// 	downstream_pressure_transducer,
	// ));

	let ads1015 = LimTemperature::new(ads1x1x::SlaveAddr::Default);
	tokio::spawn(demo::read_ads1015(ads1015));

	let wheel_encoder = WheelEncoder::new();
	tokio::spawn(demo::read_wheel_encoder(wheel_encoder));

	let gyro: Gyroscope = Gyroscope::new();
	tokio::spawn(demo::read_gyroscope(gyro));
	let mut brakes = Brakes::new();
	brakes.disengage();
	// tokio::spawn(demo::brake(brakes));

	let high_voltage_system = HighVoltageSystem::new();
	tokio::spawn(demo::high_voltage_system(high_voltage_system));

	// let lidar = Lidar::new();
	// tokio::spawn(demo::read_lidar(lidar));

	let limcurrent = LimCurrent::new(ads1x1x::SlaveAddr::Default);
	tokio::spawn(demo::read_lim_current(limcurrent));

	// let inverter_board = InverterBoard::new();
	// tokio::spawn(demo::inverter_control(inverter_board));

	// let motors = Motors::new("/dev/ttyACM0");
	// tokio::spawn(demo::vesc_motors(motors));

	// let mut state_machine = StateMachine::new(io);
	// tokio::spawn(async move {
	// 	state_machine.run().await;
	// });

	let app = Router::new().layer(layer); // Keep your existing SocketIo layer

	info!("Starting server on port 5000");

	info!("Starting server on port 5000");

	let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}

	Ok(())
}
