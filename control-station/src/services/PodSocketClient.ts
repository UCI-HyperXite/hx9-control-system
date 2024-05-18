import { Dispatch, SetStateAction } from "react";
import { Socket } from "socket.io-client";
import { ioNamespace } from "./socketHandler";

export enum State {
	Init = "Init",
	Load = "Load",
	Running = "Running",
	Stop = "Stop",
	Halt = "Halt",
}

interface ServerToClientEvents {
	connect: () => void;
	disconnect: (reason: Socket.DisconnectReason) => void;
	serverResponse: (data: string) => void;
}

interface ClientToServerEvents {
	load: (ack: (data: string) => void) => void;
	run: (ack: (data: string) => void) => void;
	stop: (ack: (data: string) => void) => void;
	halt: (ack: (data: string) => void) => void;
}

export interface PodData {
	connected: boolean;
	state: State;
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
			serverResponse: this.onData.bind(this),
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

	sendLoad(): void {
		this.socket.emit("load", (response: string) => {
			console.log("Server acknowledged:", response);
		});
		this.setPodData((d) => ({ ...d, state: State.Load }));
	}

	sendRun(): void {
		this.socket.emit("run", (response: string) => {
			console.log("Server acknowledged:", response);
		});
		this.setPodData((d) => ({ ...d, state: State.Running }));
	}

	sendStop(): void {
		this.socket.emit("stop", (response: string) => {
			console.log("Server acknowledged:", response);
		});
		this.setPodData((d) => ({ ...d, state: State.Stop }));
	}

	sendHalt(): void {
		this.socket.emit("halt", (response: string) => {
			console.log("Server acknowledged:", response);
		});
		this.setPodData((d) => ({ ...d, state: State.Halt }));
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
}

export default PodSocketClient;
