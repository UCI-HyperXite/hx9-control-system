use tracing::info;

use crate::components::brakes::Brakes;
use crate::components::gyro::Gyroscope;
use crate::components::high_voltage_system::HighVoltageSystem;
use crate::components::lim_temperature::LimTemperature;
use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;
use crate::components::wheel_encoder::WheelEncoder;
//use log::info;
use tokio::time;

pub async fn blink(mut signal_light: SignalLight) {
	let mut i = 0;

	info!("Starting blink demo.");
	loop {
		tokio::time::sleep(std::time::Duration::from_millis(500)).await;
		if i % 4 == 0 {
			signal_light.enable();
		} else if i % 4 == 1 {
			signal_light.disable();
		}

		i += 1;
	}
}

pub async fn read_pressure_transducer(mut pressure_transducer: PressureTransducer) {
	info!("Starting pressure transducer demo.");

	loop {
		tokio::time::sleep(std::time::Duration::new(1, 0)).await;
		println!("{:?}", pressure_transducer.read_pressure());
	}
}

pub async fn read_ads1015(mut lim_temperature: LimTemperature) {
	info!("Starting ADS1015 Demo.");

	let mut i = 0;
	loop {
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
		println!("{:?}", lim_temperature.read_lim_temps());
		i += 1;
		if i > 1000 {
			break;
		}
	}

	lim_temperature.cleanup();
}

pub async fn read_gyroscope(mut gyroscope: Gyroscope) {
	info!("Starting Gyroscope Demo.");
	tokio::spawn(async move {
		loop {
			let orientation = gyroscope.read_orientation();
			tokio::time::sleep(std::time::Duration::from_millis(100)).await;
			println!(
				"Pitch: {:?}, Roll: {:?}",
				orientation.pitch, orientation.roll
			);
		}
	});
}

pub async fn read_wheel_encoder(mut wheel_encoder: WheelEncoder) {
	info!("Starting wheel encoder demo.");
	loop {
		let (speed, distance) = wheel_encoder.measure().expect("Failed to measure speed and distance");
		println!("Speed: {:.2} m/s, Distance: {:.2} m", speed, distance);
		tokio::time::sleep(std::time::Duration::new(1, 0)).await;
	}
}

pub async fn brake(mut brakes: Brakes) {
	let mut i = 0;

	info!("Starting brakes demo.");
	loop {
		tokio::time::sleep(std::time::Duration::from_secs(30)).await;
		if i % 4 == 0 {
			brakes.engage();
		} else if i % 4 == 1 {
			brakes.disengage();
		}

		i += 1;
	}
}

pub async fn high_voltage_system(mut high_voltage_system: HighVoltageSystem) {
	let mut i = 0;

	info!("Starting high voltage system demo.");
	loop {
		tokio::time::sleep(std::time::Duration::from_secs(30)).await;
		if i % 4 == 0 {
			high_voltage_system.enable();
		} else if i % 4 == 1 {
			high_voltage_system.disable();
		}

		i += 1;
	}
}
