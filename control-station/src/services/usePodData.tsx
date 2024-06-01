import { useEffect, useMemo, useState } from "react";
import PodSocketClient, { PodData, State } from "./PodSocketClient";

function usePodData() {
	const [podData, setPodData] = useState<PodData>({
		state: State.Disconnected,
		connected: false,
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
