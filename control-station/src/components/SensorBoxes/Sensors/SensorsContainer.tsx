import LargeBox from "./LargeBox";
import SensorBox from "./SensorBox";

function SensorContainer() {
	return (
		<div className="SensorContainer">
			<SensorBox />
			<SensorBox />
			<SensorBox />
			<SensorBox />
			<LargeBox />
		</div>
	);
}

export default SensorContainer;
