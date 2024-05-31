import { ControlPanel, Navbar, SensorData, StatusIndicator } from "@/components";
import usePodData from "./services/usePodData";
import { createContext } from "react";
import { PodData } from "./services/PodSocketClient";

const { podData, podSocketClient } = usePodData();
export const podContext = createContext<PodData>(podData);

function App() {
	return (
		<main>
			<Navbar />
			<podContext.Provider value={podData}>
				<SensorData />
			</podContext.Provider>
			<StatusIndicator state={podData.state} />
			<ControlPanel podSocketClient={podSocketClient} />
		</main>
	);
}

export default App;
