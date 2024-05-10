import "./SensorBox.css";

import React from "react";

interface SensorBoxProps {
	sensor_value: number;
}

const SensorBox: React.FC<SensorBoxProps> = ({ sensor_value }) => {
	return (
		<div className="sensorbox">
			<h3 style={{ textAlign: "center", height: "10%" }}>Title</h3>
			<p className="sensor-value">{sensor_value}</p>
		</div>
	);
};

export default SensorBox;
