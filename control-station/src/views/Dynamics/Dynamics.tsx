import SensorBox from "@/components/SensorBoxes/Sensors/SensorBox";
import PodContext from "@/services/PodContext";
import FrontPod from "@/data/images/FrontPod.png";
import SidePod from "@/data/images/SidePod.png";
import "./Dynamics.css";
import { useContext } from "react";

function Dynamics() {
	const { podData } = useContext(PodContext);
	const { gyroscope } = podData;
	return (
		<div className="dynamics">
			<div className="dynamics-sensorbox">
				<SensorBox title="Roll" value={Math.round(gyroscope.roll * 100) / 100} />
				<SensorBox title="Pitch" value={Math.round(gyroscope.pitch * 100) / 100} />
			</div>
			<div className="dynamics-pictures">
				<div className="pod-picture-container">
					<img
						src={FrontPod}
						alt="front view of the pod"
						style={{
							transform: `rotate(${Math.round(gyroscope.roll * 100) / 100}deg)`,
						}}
					/>
				</div>
				<div className="pod-picture-container">
					<img
						src={SidePod}
						alt="side view of the pod"
						style={{
							transform: `rotate(${Math.round(gyroscope.pitch * 100) / 100}deg)`,
						}}
					/>
				</div>
			</div>
		</div>
	);
}

export default Dynamics;
