import SensorBox from "./SensorBox";
import usePodData from "@/services/usePodData";

function SensorContainer() {
	const { podSocketClient } = usePodData();
	return (
		<div className="SensorContainer">
			<SensorBox sensor_value={podSocketClient.getData()?.pt1} />
			<SensorBox sensor_value={podSocketClient.getData()?.pt2} />
			<SensorBox sensor_value={0} />
			<SensorBox sensor_value={0} />
			<SensorBox sensor_value={0} />
			<SensorBox sensor_value={0} />
		</div>
	);
}

export default SensorContainer;
