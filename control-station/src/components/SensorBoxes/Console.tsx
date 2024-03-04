import "./SensorData.css";

function Console() {
	return (
		<div className="console">
			<h1>Console</h1>
			<ul className="console-list">
				<li className="console-list-item">Start Sent</li>
				<li className="console-list-item">Stop Sent</li>
				<li className="console-list-item">Load Sent</li>
				<li className="console-list-item">Force Stop Sent</li>
			</ul>
		</div>
	);
}

export default Console;
