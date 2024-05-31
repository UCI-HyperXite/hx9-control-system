import { useContext } from "react";
import SensorBox from "./SensorBox";
import PodContext from "@/services/PodContext";

function SensorContainer() {
	const { podData } = useContext(PodContext);
	return (
		<div className="SensorContainer">
			<SensorBox title="Speed" value={podData.gyroscope} />
			<SensorBox title="Distance" value={podData.wheel_encoder} />
			<SensorBox title="PT1" value={podData.downstream_pressure_transducer} />
			<SensorBox title="PT2" value={podData.upstream_pressure_transducer} />
		</div>
	);
}

export default SensorContainer;
