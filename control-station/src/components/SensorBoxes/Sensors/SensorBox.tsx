import "./SensorBox.css";

interface Value {
	value: number;
}

function SensorBox({ value }: Value) {
	return (
		<div className="sensorbox">
			<h3 style={{ textAlign: "center", height: "10%" }}>Title</h3>
			<p className="sensor-value">{value}</p>
		</div>
	);
}

export default SensorBox;
