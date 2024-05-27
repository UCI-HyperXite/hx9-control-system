import { ControlPanel, Navbar, SensorData, StatusIndicator } from "@/components";
import usePodData from "./services/usePodData";

function App() {
	const { podData, podSocketClient } = usePodData();

	return (
		<main>
			<Navbar />
			<SensorData />
			<StatusIndicator state={podData.state} />
			<ControlPanel podSocketClient={podSocketClient} />
		</main>
	);
}

export default App;
