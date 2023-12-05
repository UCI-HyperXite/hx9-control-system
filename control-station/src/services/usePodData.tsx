import { useEffect, useMemo, useState } from "react";
import PodSocketClient, { PodData } from "./PodSocketClient";

function usePodData() {
	const [podData, setPodData] = useState<PodData>({
		connected: false,
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
