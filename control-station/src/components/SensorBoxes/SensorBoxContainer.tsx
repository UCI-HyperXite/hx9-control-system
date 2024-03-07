import SensorContainer from "./Sensors/SensorsContainer";
import "./sensorboxct.css";
import Camera from "./Camera";
import Console from "./Console";
export default function SensorBoxContainer() {
	return (
		<div className="sensorboxcontainer">
			<SensorContainer />
			<div style={{ width: "50%", height: "80vh" }}>
				<Camera />
				<Console />
			</div>
		</div>
	);
}
