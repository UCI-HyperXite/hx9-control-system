import React, { createContext, useEffect, useState } from "react";
import Navbar from "@/components/Navbar/Navbar";
import SensorBoxContainer from "@/components/SensorBoxes/SensorBoxContainer";
import ControlPanel from "@/components/ControlPanel/ControlPanel";
import socket from "@/services/SocketFunctions";

interface ConsoleContextProps {
	consoleData: number[];
	setConsoleData: React.Dispatch<React.SetStateAction<number[]>>;
	sensorData: SensorData;
	setSensorData: React.Dispatch<React.SetStateAction<SensorData>>;
}

interface SensorData {
	speed: number;
	distance: number;
	hdeg: number;
	vdeg: number;
}

export const consoleContext = createContext<ConsoleContextProps | undefined>(
	undefined,
);

const App: React.FC = () => {
	const [consoleData, setConsoleData] = useState<number[]>([]);
	const [sensorData, setSensorData] = useState<SensorData>({
		speed: 0,
		distance: 0,
		hdeg: 0,
		vdeg: 0,
	});
	useEffect(() => {
		socket.on("gyro_data", (data) => {
			console.log("received");
			setSensorData((prevData) => {
				return {
					...prevData,
					hdeg: data.pitch,
					vdeg: data.roll,
				};
			});
		});
	}, [socket, setSensorData]);
	

	return (
		<consoleContext.Provider
			value={{ consoleData, setConsoleData, sensorData, setSensorData }}
		>
			<main>
				<Navbar />
				<SensorBoxContainer />
				<ControlPanel />
			</main>
		</consoleContext.Provider>
	);
};

export default App;
