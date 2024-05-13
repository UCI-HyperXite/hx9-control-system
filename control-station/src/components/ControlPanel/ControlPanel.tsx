import "./ControlPanel.css";
import usePodData from "@/services/usePodData";

function ControlPanel() {
	const { podSocketClient } = usePodData();
	return (
		<div className="controlpanel">
			<button className="button run" onClick={() => podSocketClient.sendRun()}>
				Run
			</button>
			<button className="button stop" onClick={() => podSocketClient.sendStop()}>
				Stop
			</button>
			<button className="button halt" onClick={() => podSocketClient.sendHalt()}>
				Halt
			</button>
			<button className="button load" onClick={() => podSocketClient.sendLoad()}>
				Load
			</button>
		</div>
	);
}

export default ControlPanel;
