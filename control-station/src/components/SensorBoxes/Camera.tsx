import "./sensorboxct.css";
import Lidar from "@/data/assets/Lidar.png";
export default function Camera() {
	return (
		<div className="camera" style={{ position: "relative" }}>
			<h1
				style={{
					color: "white",
					position: "absolute",
					right: "10px",
					top: "0",
					fontSize: "1.1rem",
				}}
			>
				Distance
			</h1>
			<img src={Lidar}></img>
		</div>
	);
}
