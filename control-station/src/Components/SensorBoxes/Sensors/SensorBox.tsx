import "./sensorbox.css";
export default function SensorBox(prop: any) {
	return (
		<div className="sensorbox">
			<h1 style={{ textAlign: "center", height: "10%" }}>{prop.title}</h1>
			<h1 className="sensor-value">{prop.value}</h1>
		</div>
	);
}
