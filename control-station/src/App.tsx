import { ControlPanel, Navbar } from "@/components";
import usePodData from "./services/usePodData";
import PodContext from "./services/PodContext";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Dashboard, Dynamics } from "@/views";

function App() {
	const { podData, podSocketClient } = usePodData();

	return (
		<main>
			<PodContext.Provider value={{ podData, podSocketClient }}>
				<Navbar />
				<BrowserRouter>
					<Routes>
						<Route path="/" element={<Dashboard />} />
						<Route path="/dynamics" element={<Dynamics />} />
					</Routes>
				</BrowserRouter>
				<ControlPanel />
			</PodContext.Provider>
		</main>
	);
}

export default App;
