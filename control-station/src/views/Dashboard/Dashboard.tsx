import { Status } from "@/components";
import usePodData from "@/services/usePodData";

function Dashboard() {
	const { podData, podSocketClient } = usePodData();

	return (
		<div>
			<h1>Dashboard</h1>
			<Status />
			<p>{podData.connected ? "connected" : "disconnected"}</p>
			<button onClick={() => podSocketClient.sendStop()}>Send Stop</button>
			<button onClick={() => podSocketClient.sendForcestop()}>Send Forcestop</button>
			<button onClick={() => podSocketClient.sendLoad()}>Send Load</button>
			<button onClick={() => podSocketClient.sendStart()}>Send Start</button>
		</div>
	);
}

export default Dashboard;
