import { useContext, useEffect, useState } from "react";
import "./SensorData.css";
import PodContext from "@/services/PodContext";

function Console() {
	const { podData } = useContext(PodContext);
	const [stateList, setStateList] = useState<string[]>([]);

	useEffect(() => {
		if (podData.state) {
			setStateList((prev) => [...prev, podData.state]);
		}
	}, [podData.state]);

	return (
		<div className="console">
			<h2>Console</h2>
			<ul className="console-list">
				{stateList.map((prop, index) => (
					<li key={index} className="console-list-item">
						{prop} State
					</li>
				))}
				<li className="console-list-item">Start Sent</li>
				<li className="console-list-item">Stop Sent</li>
				<li className="console-list-item">Load Sent</li>
				<li className="console-list-item">Force Stop Sent</li>
			</ul>
		</div>
	);
}

export default Console;
