import { useContext } from "react";

import PodContext from "@/services/PodContext";
import { State } from "@/services/PodSocketClient";

import "./StatusIndicator.css";

function StatusIndicator() {
	const { podData } = useContext(PodContext);
	const { state } = podData;

	return (
		<div className="sensorbox status-indicator" style={{ fontSize: "1rem" }}>
			{Object.values(State).map((s) => {
				return (
					<div key={s} className={`group ${s.toLowerCase()}-state`}>
						<span className={`circle` + (s === state ? " active" : "")}></span>
						<br />
						<div className="state-text">{s}</div>
					</div>
				);
			})}
		</div>
	);
}

export default StatusIndicator;
