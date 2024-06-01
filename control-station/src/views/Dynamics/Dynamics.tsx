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
				<SensorBox title="Roll" value={gyroscope.roll} />
				<SensorBox title="Pitch" value={gyroscope.pitch} />
			</div>
			<div className="dynamics-pictures">
				<div className="pod-picture-container">
					<img src={FrontPod} />
				</div>
				<div className="pod-picture-container">
					<img src={SidePod} />
				</div>
			</div>
		</div>
	);
}

export default Dynamics;
