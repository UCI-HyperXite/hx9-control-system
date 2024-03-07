import { useContext } from "react";
import { consoleContext } from "@/App";
import "./sensorboxct.css";

export default function Console() {
	const context = useContext(consoleContext);

	if (!context) {
		return <div>Error: Console context not available</div>;
	}

	const { consoleData } = context;

	return (
		<div className="console">
			<h1>Console</h1>
			<ul className="console-list">
				{consoleData.map((item: number, index: number) => (
					<li key={index} className="console-list-item">
						{item === 1
							? "Start Sent"
							: item === 0
								? "Stop Sent"
								: item === 2
									? "Force Stop Sent"
									: item === 3
										? "Load Sent"
										: ""}
					</li>
				))}
			</ul>
		</div>
	);
}
