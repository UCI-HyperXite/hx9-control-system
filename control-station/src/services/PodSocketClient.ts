import { Dispatch, SetStateAction } from "react";
import { Socket } from "socket.io-client";
import { ioNamespace } from "./socketHandler";

export enum State {
	Disconnected = "Disconnected",
	Init = "Init",
	Load = "Load",
	Running = "Running",
	Stopped = "Stopped",
	Halted = "Halted",
	Faulted = "Faulted",
}

interface ServerToClientEvents {
	connect: () => void;
	disconnect: (reason: Socket.DisconnectReason) => void;
	serverResponse: (data: Partial<PodData>) => void;
}

interface Message {
	timestamp: Date;
	message: string;
}

interface ClientToServerEvents {
	load: (ack: (data: string) => void) => void;
	run: (ack: (data: string) => void) => void;
	stop: (ack: (data: string) => void) => void;
	halt: (ack: (data: string) => void) => void;
}

interface WheelEncoder {
	distance: number;
	velocity: number;
}

interface Gyroscope {
	pitch: number;
	roll: number;
}

export interface PodData {
	connected: boolean;
	state: State;
	gyroscope: Gyroscope;
	wheel_encoder: WheelEncoder;
	downstream_pressure_transducer: number;
	upstream_pressure_transducer: number;
	lim_temperature_port: number;
	lim_temperature_starboard: number;
	messages: Message[];
	lidar: number;
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
			this.addMessage(response, State.Load);
		});
	}

	sendRun(): void {
		this.socket.emit("run", (response: string) => {
			console.log("Server acknowledged:", response);
			this.addMessage(response, State.Running);
		});
	}

	sendStop(): void {
		this.socket.emit("stop", (response: string) => {
			console.log("Server acknowledged:", response);
			this.addMessage(response, State.Stopped);
		});
	}

	sendHalt(): void {
		this.socket.emit("halt", (response: string) => {
			console.log("Server acknowledged:", response);
			this.addMessage(response, State.Halted);
		});
	}

	// TODO: On connecting, the state below should be what's provided by the pod
	// if it's already running. Otherwise, the states should be State.Init
	private onConnect(): void {
		console.log("Connected to server as", this.socket.id);
		this.setPodData((d) => ({ ...d, connected: true, state: State.Init }));
	}

	private onDisconnect(reason: Socket.DisconnectReason): void {
		console.log(`Disconnected from server: ${reason}`);
		this.setPodData((d) => ({ ...d, connected: false, state: State.Disconnected }));
	}

	private onData(data: Partial<PodData>): void {
		console.log("server says", data);
		this.setPodData((d) => ({ ...d, ...data }));
	}

	private addMessage(response: string, newState: State): void {
		const timestamp = new Date();
		const newMessage = {
			timestamp,
			message: response,
		};

		this.setPodData((d) => ({
			...d,
			state: newState,
			messages: [...d.messages, newMessage],
		}));
	}
}

export default PodSocketClient;
