import SensorBox from "./SensorBox";

export default function SensorContainer() {
	return (
		<div className="SensorContainer">
			<SensorBox title="Speed" value="247" />
			<SensorBox title="Distance" value="132" />
			<SensorBox title="Acceleration" value="343" />
			<SensorBox title="Input Current" value="5" />
			<SensorBox title="Input Voltage" value="50" />
			<SensorBox title="Temperature" value="68" />
		</div>
	);
}
