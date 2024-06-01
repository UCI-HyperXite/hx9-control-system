import "./SensorData.css";
import { LineChart } from "@mui/x-charts";
import PodContext from "@/services/PodContext";
import { useContext, useEffect, useState } from "react";

function Camera() {
	const { podData } = useContext(PodContext);
	const [lidarList, setLidarList] = useState<number[]>([]);

	useEffect(() => {
		setLidarList((prevLidarList) => {
			if (prevLidarList.length < 200) {
				return [...prevLidarList, podData.lidar];
			}
			return [...prevLidarList.slice(1), podData.lidar];
		});
	}, [podData.lidar]);

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
