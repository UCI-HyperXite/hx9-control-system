import FrontPod from "@/data/assets/FrontPod.png";
import SidePod from "@/data/assets/SidePod.png";
export default function LargeBox(prop: any) {
	return (
		<div className="largebox">
			<img
				className="sensor-pic"
				src={prop.pic == "Front" ? FrontPod : SidePod}
				style={{ rotate: `${prop.rotate}` }}
			/>
			<h1>{prop.rotate}</h1>
		</div>
	);
}
