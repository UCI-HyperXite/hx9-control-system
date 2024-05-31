import { useContext } from "react";
import SensorBox from "./SensorBox";
import PodContext from "@/services/PodContext";
import StatusIndicator from "@/components/StatusIndicator/StatusIndicator";
function SensorContainer() {
	const { podData } = useContext(PodContext);
	return (
		<div className="SensorContainer">
			<SensorBox title="Speed" value={podData.gyroscope} />
			<SensorBox title="Distance" value={podData.wheel_encoder} />
			<SensorBox title="PT1" value={podData.downstream_pressure_transducer} />
			<SensorBox title="PT2" value={podData.upstream_pressure_transducer} />
			<SensorBox title="Lim Current" value={podData.upstream_pressure_transducer} />
			<StatusIndicator />
		</div>
	);
}

export default SensorContainer;
