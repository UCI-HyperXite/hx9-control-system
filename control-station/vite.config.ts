import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
	resolve: {
		alias: {
			"@/": "/src/",
			"@Navbar":"/src/components/navbar/",
			"@SensorBoxes":"/src/components/sensorboxes/",
			"@ControlPanel":"/src/components/controlpanel/",
			"$/":"/public/"
		},
	},
});
