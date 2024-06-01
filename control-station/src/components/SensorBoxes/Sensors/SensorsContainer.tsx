import { useContext } from "react";
import SensorBox from "./SensorBox";
import PodContext from "@/services/PodContext";
import StatusIndicator from "@/components/StatusIndicator/StatusIndicator";
function SensorContainer() {
	const { podData } = useContext(PodContext);
	const {
		wheel_encoder,
		downstream_pressure_transducer,
		upstream_pressure_transducer,
		lim_temperature_port,
	} = podData;
	return (
		<div className="SensorContainer">
			<SensorBox title="Speed" value={wheel_encoder.velocity} />
			<SensorBox title="Distance" value={wheel_encoder.distance} />
			<SensorBox title="Downstream PT" value={downstream_pressure_transducer} />
			<SensorBox title="Upstream PT" value={upstream_pressure_transducer} />
			<SensorBox title="Breaking Distance" value={lim_temperature_port} />
			<StatusIndicator />
		</div>
	);
}

export default SensorContainer;
