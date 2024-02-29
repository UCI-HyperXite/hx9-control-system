import React, { createContext, useState, useEffect } from "react";
import Navbar from "@/Components/Navbar/Navbar";
import SensorBoxContainer from "@/Components/SensorBoxes/SensorBoxContainer";
import ControlPanel from "@/Components/ControlPanel/ControlPanel";
import { io, Socket } from "socket.io-client";

const socket: Socket = io("http://localhost:8000");

interface ConsoleContextProps {
	consoleData: number[];
	setConsoleData: React.Dispatch<React.SetStateAction<number[]>>;
	imageData: string | null;
	setImageData: React.Dispatch<React.SetStateAction<string | null>>;
}

export const consoleContext = createContext<ConsoleContextProps | undefined>(
	undefined,
);

const App: React.FC = () => {
	const [consoleData, setConsoleData] = useState<number[]>([]);
	const [imageData, setImageData] = useState<string | null>(null);

	useEffect(() => {
		const handleImageData = (data: { image: string }) => {
			setImageData(data.image);
			console.log("Image Data:", data.image);
		};

		socket.on("image_data", handleImageData);

		return () => {
			socket.off("image_data", handleImageData);
			socket.disconnect();
		};
	}, [imageData]);

	return (
		<consoleContext.Provider
			value={{ consoleData, setConsoleData, imageData, setImageData }}
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
