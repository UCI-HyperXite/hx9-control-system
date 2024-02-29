import "./controlpanel.css";

export default function ControlPanel() {
	return (
		<div className="controlpanel">
			<button id="start" className="button">
				Start
			</button>
			<button id="stop" className="button">
				Stop
			</button>
			<button id="force" className="button">
				Force Stop
			</button>
			<button id="load" className="button">
				Load
			</button>
		</div>
	);
}
