import { useContext } from "react";
import SensorBox from "./SensorBox";
import PodContext from "@/services/PodContext";

function SensorContainer() {
	const { podData } = useContext(PodContext);
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
