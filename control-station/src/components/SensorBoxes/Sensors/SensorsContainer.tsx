import SmallBox from "./SmallBox";
import LargeBox from "./LargeBox";
import { consoleContext } from "@/App";
import { useContext } from "react";
export default function SensorContainer() {
	const context = useContext(consoleContext);
	if (!context) {
		return <div>Error: Console context not available</div>;
	}
	const { sensorData } = context;
	return (
		<div className="SensorContainer">
			<SmallBox title="Distance" value={sensorData.distance} />
			<SmallBox title="Speed" value={sensorData.speed} />
			<LargeBox pic="Front" rotate={`${sensorData.hdeg}deg`} />
			<LargeBox pic="Side" rotate={`${sensorData.vdeg}deg`} />
		</div>
	);
}
