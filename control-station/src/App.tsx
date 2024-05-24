import { ControlPanel, Navbar } from "@/components";
import { SensorData, consoleContext } from "@/interfaces/SensorData";
import { useState } from "react";

function App() {
	const [sensorData, setSensorData] = useState<SensorData>({
		currState: "Run",
	});
	return (
		<main>
			<consoleContext.Provider value={{ sensorData, setSensorData }}>
				<Navbar />
				<ControlPanel />
			</consoleContext.Provider>
		</main>
	);
}

export default App;
