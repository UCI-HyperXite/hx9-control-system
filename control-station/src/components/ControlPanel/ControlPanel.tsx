import "./ControlPanel.css";

function ControlPanel() {
	return (
		<div className="controlpanel">
			<button className="button start">Start</button>
			<button className="button stop">Stop</button>
			<button className="button force">Force Stop</button>
			<button className="button load">Load</button>
		</div>
	);
}

export default ControlPanel;
