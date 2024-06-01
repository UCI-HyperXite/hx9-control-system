import { useContext, useEffect, useRef, useState } from "react";
import "./SensorData.css";
import PodContext from "@/services/PodContext";

function Console() {
	const { podData } = useContext(PodContext);
	const [stateList, setStateList] = useState<string[]>([]);
	const listEndRef = useRef<HTMLLIElement | null>(null);

	useEffect(() => {
		if (podData.state) {
			setStateList((prev) => [...prev, podData.state]);
		}
	}, [podData.state]);

	useEffect(() => {
		if (listEndRef.current) {
			listEndRef.current.scrollIntoView({ behavior: "smooth" });
		}
	}, [stateList]);

	return (
		<div className="console">
			<h2>Console</h2>
			<ul className="console-list">
				{stateList.map((prop, index) => (
					<li
						key={index}
						className="console-list-item"
						ref={index === stateList.length - 1 ? listEndRef : null}
					>
						{prop} State
					</li>
				))}
			</ul>
		</div>
	);
}

export default Console;
