import "./SensorBox.css";

function SensorBox(prop: any) {
	return (
		<div className="sensorbox">
			<h3 style={{ textAlign: "center", height: "10%" }}>Title</h3>
			<p className="sensor-value">{prop.value}</p>
		</div>
	);
}

export default SensorBox;
