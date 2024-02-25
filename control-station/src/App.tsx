import React, { createContext, useState } from "react";
import Navbar from "@/Components/Navbar/Navbar";
import SensorBoxContainer from "@/Components/SensorBoxes/SensorBoxContainer";
import ControlPanel from "@/Components/ControlPanel/ControlPanel";
interface ConsoleContextProps {
	console: number[];
	setconsole: React.Dispatch<React.SetStateAction<number[]>>;
}

export const consoleContext = createContext<ConsoleContextProps | undefined>(
	undefined
);

const App: React.FC = () => {
	const [console, setconsole] = useState<number[]>([]);
	return (
		<consoleContext.Provider value={{ console, setconsole }}>
			<main>
				<Navbar />
				<SensorBoxContainer />
				<ControlPanel />
			</main>
		</consoleContext.Provider>
	);
};

export default App;
