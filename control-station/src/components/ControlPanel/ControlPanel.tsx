import { useContext } from "react";
import { consoleContext } from "../../App";
import "./controlpanel.css";
import socket from "@/services/SocketFunctions";

export default function ControlPanel() {
	const context = useContext(consoleContext);

	if (!context) {
		// Handle the case when context is undefined
		return <div>Error: Context not available</div>;
	}

	const { setConsoleData } = context;

	const handleStart = () => {
		setConsoleData((prev: number[]) => [...prev, 1]);
		socket.emit("start");
	};

	const handleStop = () => {
		setConsoleData((prev: number[]) => [...prev, 0]);
		socket.emit("stop");
	};

	const handleForceStop = () => {
		setConsoleData((prev: number[]) => [...prev, 2]);
		socket.emit("forcestop");
	};

	const handleLoad = () => {
		setConsoleData((prev: number[]) => [...prev, 3]);
		socket.emit("load");
	};

	return (
		<div className="controlpanel">
			<button id="start" className="button" onClick={handleStart}>
				Start
			</button>
			<button id="stop" className="button" onClick={handleStop}>
				Stop
			</button>
			<button id="force" className="button" onClick={handleForceStop}>
				Force Stop
			</button>
			<button id="load" className="button" onClick={handleLoad}>
				Load
			</button>
		</div>
	);
}
