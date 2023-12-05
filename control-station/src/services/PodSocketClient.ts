import { Dispatch, SetStateAction } from "react";
import { Socket } from "socket.io-client";
import { ioNamespace } from "./socketHandler";

interface ServerToClientEvents {
	connect: () => void;
	disconnect: () => void;
	pong: (data: string) => void;
}

interface ClientToServerEvents {
	ping: (data: string) => void;
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
		this.socket = ioNamespace("control-station");
		this.serverEvents = {
			connect: this.onConnect.bind(this),
			disconnect: this.onDisconnect.bind(this),
			pong: this.onPong.bind(this),
		} as const;
		this.setPodData = setPodData;
	}

	enable(): void {
		this.socket.connect();
		console.debug("Enabling socket event handlers");
		(Object.entries(this.serverEvents) as Entries<ServerToClientEvents>).forEach(
			([event, handler]) => {
				this.socket.on(event, handler);
			}
		);
	}

	disable(): void {
		console.debug("Disabling socket event handlers");
		Object.keys(this.serverEvents).forEach((event) => {
			this.socket.off(event as keyof ServerToClientEvents);
		});
		this.socket.disconnect();
	}

	onConnect(): void {
		console.log("Connected to server as", this.socket.id);
	}

	onDisconnect(): void {
		console.log("Disconnected from server.");
	}

	// Send a ping to the server
	sendPing(): void {
		this.socket.emit("ping", "ping");
	}

	onPong(data: string): void {
		console.log("server says", data);
	}
}

export default PodSocketClient;
