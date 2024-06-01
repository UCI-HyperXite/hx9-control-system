import "./SensorData.css";
import { LineChart } from "@mui/x-charts";
function Camera() {
	return (
		<div className="camera">
			<LineChart
				xAxis={[{ data: [1, 2, 3, 5, 8, 10] }]}
				series={[
					{
						data: [2, 5.5, 2, 8.5, 1.5, 5],
					},
				]}
				className="lidar-chart"
			/>
		</div>
	);
}

export default Camera;
