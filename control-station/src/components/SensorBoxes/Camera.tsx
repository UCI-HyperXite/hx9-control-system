import "./SensorData.css";
import { LineChart } from "@mui/x-charts";
import PodContext from "@/services/PodContext";
import { useContext, useEffect, useState } from "react";

function Camera() {
	const { podData } = useContext(PodContext);
	const [lidarList, setLidarList] = useState<number[]>([]);

	useEffect(() => {
		console.log(lidarList);
		setLidarList((prevLidarList) => [...prevLidarList, podData.lidar]);
	}, [podData, lidarList]); // Added lidarList to the dependency array

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
