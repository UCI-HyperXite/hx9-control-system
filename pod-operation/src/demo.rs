use tracing::info;

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
