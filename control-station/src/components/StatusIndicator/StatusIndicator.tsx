import { State } from "@/services/PodSocketClient";
import "./StatusIndicator.css";

interface StatusIndicatorProps {
	state: State;
}

function StatusIndicator({ state }: StatusIndicatorProps) {
	return (
		<div className="status-indicator">
			{Object.values(State).map((s) => {
				return (
					<div key={s} className={`group ${s.toLowerCase()}-state`}>
						<span className={`circle` + (s === state ? " active" : "")}></span>
						<div className="state-text">{s}</div>
					</div>
				);
			})}
		</div>
	);
}

export default StatusIndicator;
