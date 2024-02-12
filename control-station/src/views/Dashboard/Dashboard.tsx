import { Status } from "@/components";
import usePodData from "@/services/usePodData";

function Dashboard() {
	const { podData, podSocketClient } = usePodData();

	return (
		<div>
			<h1>Dashboard</h1>
			<Status />
			<p>{podData.connected ? "connected" : "disconnected"}</p>
			<button onClick={() => podSocketClient.sendPing()}>Send Ping</button>
		</div>
	);
}

export default Dashboard;
