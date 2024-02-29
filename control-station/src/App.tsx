import Navbar from "@/components/Navbar/Navbar";
import SensorBoxContainer from "@/components/SensorBoxes/SensorBoxContainer";
import ControlPanel from "@/components/ControlPanel/ControlPanel";

export default function App() {
	return (
		<main>
			<Navbar />
			<SensorBoxContainer />
			<ControlPanel />
		</main>
	);
}
