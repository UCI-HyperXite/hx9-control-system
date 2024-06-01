import { useContext, useEffect, useRef } from "react";
import "./SensorData.css";
import PodContext from "@/services/PodContext";

function Console() {
	const { podData } = useContext(PodContext);
	const listEndRef = useRef<HTMLLIElement | null>(null);

	useEffect(() => {
		if (listEndRef.current) {
			listEndRef.current.scrollIntoView({ behavior: "smooth" });
		}
		console.log(podData.messages);
	}, [podData.messages]);

	return (
		<div className="console">
			<h2>Console</h2>
			<ul className="console-list">
				{podData.messages.map((prop, index) => (
					<li
						key={index}
						className="console-list-item"
						ref={index === podData.messages.length - 1 ? listEndRef : null}
					>
						{prop.timestamp} &nbsp;
						{prop.message.toUpperCase()} State
					</li>
				))}
			</ul>
		</div>
	);
}

export default Console;
