use tracing::info;

use crate::components::lim_temperature::LimTemperature;
use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;

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
		println!("{:?}", pressure_transducer.read());
	}
}

pub async fn read_ads1015(mut lim_temperature: LimTemperature) {
	info!("Starting ADS1015 Demo.");

	let mut i = 0;
	loop {
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
		println!("{:?}", lim_temperature.read_pins());
		i += 1;
		if i > 1000 {
			break;
		}
	}

	lim_temperature.cleanup();
}
