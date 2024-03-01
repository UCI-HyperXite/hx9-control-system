import Camera from "./Camera";
import Console from "./Console";
import SensorContainer from "./Sensors/SensorsContainer";

import "./SensorData.css";

export default function SensorData() {
	return (
		<div className="sensordata">
			<SensorContainer />
			<div style={{ width: "50%" }}>
				<Camera />
				<Console />
			</div>
		</div>
	);
}
