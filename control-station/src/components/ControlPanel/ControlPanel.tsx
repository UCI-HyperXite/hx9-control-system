import "./ControlPanel.css";
import usePodData from "@/services/usePodData";

function ControlPanel() {
	const { podData, podSocketClient } = usePodData();
	return (
		<div className="controlpanel">
			<button className="button start" onClick={() => podSocketClient.sendStart()}>
				Start
			</button>
			<button className="button stop" onClick={() => podSocketClient.sendStop()}>
				Stop
			</button>
			<button className="button force" onClick={() => podSocketClient.sendForcestop()}>
				Force Stop
			</button>
			<button className="button load" onClick={() => podSocketClient.sendLoad()}>
				Load
			</button>
		</div>
	);
}

export default ControlPanel;
