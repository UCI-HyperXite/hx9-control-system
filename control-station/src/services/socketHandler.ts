import { io } from "socket.io-client";

const SERVER_URL = "";
const OPTIONS = { autoConnect: false };

export const ioNamespace = (namespace: string) => {
	return io(`${SERVER_URL}/${namespace}`, OPTIONS);
};
