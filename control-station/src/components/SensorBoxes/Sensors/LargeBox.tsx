import "./LargeBox.css";
import { useContext, useEffect, useState } from "react";
import { consoleContext } from "@/interfaces/SensorData";

export default function LargeBox() {
	const context = useContext(consoleContext);

	const [selectedState, setSelectedState] = useState("");

	useEffect(() => {
		const currState = context?.sensorData?.currState ?? "";
		setSelectedState(currState);
	}, [context]);

	if (!context) {
		return <div>Error: Console context not available</div>;
	}

	return (
		<div className="largebox">
			<ul>
				<li>
					<input
						id="run"
						className="stateradio"
						type="radio"
						disabled
						checked={selectedState === "Run"}
					/>
					<label
						style={{
							color: " rgb(35, 128, 30)",
							fontWeight: "bold",
							paddingLeft: "20px",
						}}
					>
						Run
					</label>
				</li>

				<li>
					<input
						id="halt"
						className="stateradio"
						type="radio"
						disabled
						checked={selectedState === "Halt"}
					/>
					<label
						style={{
							color: "rgb(149, 46, 46)",
							fontWeight: "bold",
							paddingLeft: "20px",
						}}
					>
						Halt
					</label>
				</li>
			</ul>
			<ul>
				<li>
					<input
						id="stop"
						className="stateradio"
						type="radio"
						disabled
						checked={selectedState === "Stop"}
					/>
					<label
						style={{
							color: "rgb(235, 63, 51)",
							fontWeight: "bold",
							paddingLeft: "20px",
						}}
					>
						Stop
					</label>
				</li>
				<li>
					<input
						id="load"
						className="stateradio"
						type="radio"
						disabled
						checked={selectedState === "Load"}
					/>
					<label
						style={{
							color: "rgb(0, 101, 188)",
							fontWeight: "bold",
							paddingLeft: "20px",
						}}
					>
						Load
					</label>
				</li>
			</ul>
		</div>
	);
}
