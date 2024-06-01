import { useEffect, useMemo, useState } from "react";
import PodSocketClient, { PodData, State } from "./PodSocketClient";

function usePodData() {
	const [podData, setPodData] = useState<PodData>({
		connected: false,
		state: State.Disconnected,
		gyroscope: { roll: 0, pitch: 0 },
		wheel_encoder: { distance: 0, velocity: 0 },
		downstream_pressure_transducer: 0,
		upstream_pressure_transducer: 0,
		lim_temperature_port: 0,
		lim_temperature_starboard: 0,
		messages: [],
		lidar: 0,
	});

	const podSocketClient = useMemo(() => new PodSocketClient(setPodData), []);

	useEffect(() => {
		podSocketClient.enable();
		// disable socket instance on cleanup
		return podSocketClient.disable.bind(podSocketClient);
	}, [podSocketClient]);

	return { podData, podSocketClient };
}

export default usePodData;
