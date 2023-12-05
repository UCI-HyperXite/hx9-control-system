import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
	resolve: {
		alias: {
			"@/": "/src/",
		},
	},
	server: {
		proxy: {
			"/socket.io": {
				target: "ws://127.0.0.1:5000",
				ws: true,
			},
		},
	},
});
