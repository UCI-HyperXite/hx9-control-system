import { createContext } from "react";

import PodSocketClient, { PodData } from "./PodSocketClient";

interface PodContext {
	podSocketClient: PodSocketClient;
	podData: Readonly<PodData>;
}

// Initialize with unusable object assuming proper values are always provided
const PodContext = createContext<PodContext>({
	podSocketClient: {} as PodSocketClient,
	podData: {} as PodData,
});

export default PodContext;
