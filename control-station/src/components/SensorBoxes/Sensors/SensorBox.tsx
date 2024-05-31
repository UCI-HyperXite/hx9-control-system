import "./SensorBox.css";

interface SensorBoxProps {
	title: string;
	value: number;
}

function SensorBox({ title, value }: SensorBoxProps) {
	return (
		<div className="sensorbox">
			<h3 style={{ textAlign: "center", height: "10%" }}>{title}</h3>
			<p className="sensor-value">{value}</p>
		</div>
	);
}

export default SensorBox;
