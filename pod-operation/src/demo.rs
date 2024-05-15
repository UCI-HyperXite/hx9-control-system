use tracing::info;

use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;
use crate::components::wheel_encoder::WheelEncoder;

pub async fn _blink(mut signal_light: SignalLight) {
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
		println!("{:?}", pressure_transducer.read());
	}
}

pub async fn read_wheel_encoder() {
	info!("Starting wheel encoder demo.");
	let mut wheel_encoder = WheelEncoder::new();
	loop {
		println!(
			"{:?}{:?}",
			wheel_encoder.read(),
			wheel_encoder.get_velocity()
		);
		tokio::time::sleep(std::time::Duration::new(1, 0)).await;
	}
}
