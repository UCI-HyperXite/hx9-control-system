import "./SensorData.css";
import { LineChart } from "@mui/x-charts";
import PodContext from "@/services/PodContext";
import { useContext, useEffect, useState } from "react";

// Ensure PodData type is imported or defined
// import { PodData } from "@/path/to/PodData";

function Camera() {
	const { podData } = useContext(PodContext);
	const [lidarList, setLidarList] = useState<number[]>([]);

	useEffect(() => {
		setLidarList((prevLidarList) => [...prevLidarList, podData.lidar]);
	}, [podData]);

	return (
		<div className="camera">
			<LineChart
				series={[
					{
						data: lidarList,
					},
				]}
				className="lidar-chart"
			/>
		</div>
	);
}

export default Camera;
