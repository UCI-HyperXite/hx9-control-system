import { io, Socket } from "socket.io-client";

const socket: Socket = io("http://localhost:3001");

socket.on("connect", () => {
	console.log("Connected");
});

export default socket;
