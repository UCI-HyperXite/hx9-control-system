import ControlPanel from "./components/ControlPanel/ControlPanel";
import Navbar from "./components/Navbar/Navbar";
import SensorBoxContainer from "./components/SensorBoxes/SensorBoxContainer";

function App() {
	return (
		<main>
			<Navbar />
			<SensorBoxContainer />
			<ControlPanel />
		</main>
	);
}

export default App;
