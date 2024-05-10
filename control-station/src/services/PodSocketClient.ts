import { Dispatch, SetStateAction } from "react";
import { Socket } from "socket.io-client";
import { ioNamespace } from "./socketHandler";

let data_array: { pt1: number; pt2: number };

interface ServerToClientEvents {
	connect: () => void;
	disconnect: (reason: Socket.DisconnectReason) => void;
	pong: (data: string) => void;
	greet: (data: string) => void;
	stop: (data: string) => void;
	forcestop: (data: string) => void;
	load: (data: string) => void;
	sensor_data: (data: string) => void;
	start: (data: string) => void;
}

interface ClientToServerEvents {
	ping: (data: string, ack: (data: string) => void) => void;
	greet: (data: string, ack: (data: string) => void) => void;
	stop: (data: string, ack: (data: string) => void) => void;
	forcestop: (data: string, ack: (data: string) => void) => void;
	load: (data: string, ack: (data: string) => void) => void;
	sensor_data: (data: string, ack: (data: string) => void) => void;
	start: (data: string, ack: (data: string) => void) => void;
}

export interface PodData {
	connected: boolean;
}

type SetPodData = Dispatch<SetStateAction<PodData>>;

// Not entirely safe to use but better than casting with `as`
// From https://stackoverflow.com/a/60142095
type Entries<T> = {
	[K in keyof T]: [K, T[K]];
}[keyof T][];

class PodSocketClient {
	socket: Socket<ServerToClientEvents, ClientToServerEvents>;
	serverEvents: ServerToClientEvents;
	setPodData: SetPodData;

	constructor(setPodData: SetPodData) {
		this.socket = ioNamespace("");
		this.serverEvents = {
			connect: this.onConnect.bind(this),
			disconnect: this.onDisconnect.bind(this),
			pong: this.onData.bind(this),
			greet: this.onData.bind(this),
			stop: this.onData.bind(this),
			forcestop: this.onData.bind(this),
			load: this.onData.bind(this),
			sensor_data: this.onSend_data.bind(this),
			start: this.onData.bind(this),
		} as const;
		this.setPodData = setPodData;
	}

	enable(): void {
		this.socket.connect();
		console.debug("Enabling socket event handlers");
		(Object.entries(this.serverEvents) as Entries<ServerToClientEvents>).forEach(
			([event, handler]) => {
				this.socket.on(event, handler);
			},
		);
	}

	disable(): void {
		console.debug("Disabling socket event handlers");
		Object.keys(this.serverEvents).forEach((event) => {
			this.socket.off(event as keyof ServerToClientEvents);
		});
		this.socket.disconnect();
	}

	// Send a ping to the server
	sendPing(): void {
		this.socket.emit("ping", "ping", (ack: string) => {
			console.log(`Server acknowledges ping with ${ack}`);
		});
	}

	sendGreet(): void {
		this.socket.emit("greet", "Hello from client", (ack: string) => {
			console.log(`Server responds to greet with ${ack}`);
		});
	}

	sendStop(): void {
		this.socket.emit("stop", "Hello from client", (ack: string) => {
			console.log(`Server responds to stop with ${ack}`);
		});
	}

	sendForcestop(): void {
		this.socket.emit("forcestop", "Hello from client", (ack: string) => {
			console.log(`Server responds to stop with ${ack}`);
		});
	}

	sendLoad(): void {
		this.socket.emit("load", "Hello from client", (ack: string) => {
			console.log(`Server responds to stop with ${ack}`);
		});
	}

	sendStart(): void {
		this.socket.emit("start", "Hello from client", (ack: string) => {
			console.log(`Server responds to stop with ${ack}`);
		});
	}

	private onConnect(): void {
		console.log("Connected to server as", this.socket.id);
		this.setPodData((d) => ({ ...d, connected: true }));
	}

	private onDisconnect(reason: Socket.DisconnectReason): void {
		console.log(`Disconnected from server: ${reason}`);
		this.setPodData((d) => ({ ...d, connected: false }));
	}

	private onData(data: string): void {
		console.log("server says", data);
	}

	private onSend_data(data: string): void {
		console.log("server says", JSON.parse(data));
	}

	getData(): { pt1: number; pt2: number } {
		return data_array;
	}
}

export default PodSocketClient;
