import { useContext } from "react";
import SmallBox from "../SensorBoxes/Sensors/SmallBox";
import { consoleContext } from "@/App";
import { BarChart, LineChart } from "@mui/x-charts";
export default function Propulsion() {
	const context = useContext(consoleContext);
	if (!context) {
		return <div>Error: Console context not available</div>;
	}
	const { sensorData } = context;
	return (
		<div style={{ display: "flex" }}>
			<div style={{ width: "50%" }}>
				<SmallBox title="Power Factor" value={sensorData.hdeg} />
				<SmallBox title="Wattmeters" value={sensorData.vdeg} />
				<SmallBox title="Input Voltage" value={sensorData.vdeg} />
			</div>
			<div style={{ width: "50%" }}>
				<div className="chart-container">
					<BarChart
						xAxis={[
							{
								scaleType: "band",
								data: ["Input Current", "Coil Temperature", "Frequency"],
							},
						]}
						series={[{ data: [4, 3, 2] }]}
					/>
				</div>
				<div className="chart-container">
					<LineChart
						xAxis={[{ data: [1, 2, 3, 5, 8, 10] }]}
						series={[
							{
								data: [2, 5.5, 2, 8.5, 1.5, 5],
							},
						]}
						height={400}
					/>
				</div>
			</div>
		</div>
	);
}
