import { ControlPanel, Navbar } from "@/components";
import usePodData from "./services/usePodData";
import PodContext from "./services/PodContext";
import { Dashboard } from "@/views";

function App() {
	const { podData, podSocketClient } = usePodData();

	return (
		<main>
			<PodContext.Provider value={{ podData, podSocketClient }}>
				<Navbar />
				<Dashboard />
				<ControlPanel />
			</PodContext.Provider>
		</main>
	);
}

export default App;
