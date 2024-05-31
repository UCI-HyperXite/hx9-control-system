import SensorBox from "./SensorBox";
import usePodData from "@/services/usePodData";

function SensorContainer() {
	const { podData } = usePodData();
	return (
		<div className="SensorContainer">
			<SensorBox value={podData.gyroscope} />
			<SensorBox value={podData.wheel_encoder} />
			<SensorBox value={podData.wheel_encoder} />
			<SensorBox value={podData.downstream_pressure_transducer} />
		</div>
	);
}

export default SensorContainer;
