use tracing::info;

use crate::components::pressure_transducer::PressureTransducer;
use crate::components::signal_light::SignalLight;

pub async fn blink(mut signal_light: SignalLight) {
	let mut i = 0;

	let mut pt = PressureTransducer::new(0x40);

	info!("Starting blink demo.");
	loop {
		tokio::time::sleep(std::time::Duration::from_millis(500)).await;
		if i % 4 == 0 {
			signal_light.enable();
		} else if i % 4 == 1 {
			signal_light.disable();
		}

		println!("Current reading: {:?}", pt.read());

		i += 1;
	}
}
