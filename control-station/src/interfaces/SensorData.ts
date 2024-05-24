import { createContext } from "react";
export interface SensorData {
	currState: string;
}

export interface ConsoleContextProps {
	sensorData: SensorData;
	setSensorData: React.Dispatch<React.SetStateAction<SensorData>>;
}

export const consoleContext = createContext<ConsoleContextProps | undefined>(undefined);
