import { io } from "socket.io-client";

const SERVER_URL = "https://glsjd1ck-5000.usw3.devtunnels.ms/";
const OPTIONS = { autoConnect: false };

export const ioNamespace = (namespace: string) => {
	return io(`${SERVER_URL}/${namespace}`, OPTIONS);
};
