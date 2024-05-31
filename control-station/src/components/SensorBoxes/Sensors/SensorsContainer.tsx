import SensorBox from "./SensorBox";
import { useContext } from "react";
import { podContext } from "@/App";

function SensorContainer() {
	const podData = useContext(podContext);

	if (!podData) {
		return <div>Loading...</div>;
	}

	return (
		<div className="SensorContainer">
			<SensorBox value={podData.gyroscope} />
			<SensorBox value={podData.wheel_encoder} />
			<SensorBox value={podData.downstream_pressure_transducer} />
			<SensorBox value={podData.upstream_pressure_transducer} />
		</div>
	);
}

export default SensorContainer;
