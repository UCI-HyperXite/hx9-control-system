// vite.config.ts
import react from "file:///home/vrushanganand/Desktop/hx9-control-system/control-station/node_modules/@vitejs/plugin-react-swc/index.mjs";
import { defineConfig } from "file:///home/vrushanganand/Desktop/hx9-control-system/control-station/node_modules/vite/dist/node/index.js";
var vite_config_default = defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@/": "/src/"
    }
  },
  server: {
    proxy: {
      "/socket.io": {
        target: "ws://127.0.0.1:5000",
        ws: true
      }
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS92cnVzaGFuZ2FuYW5kL0Rlc2t0b3AvaHg5LWNvbnRyb2wtc3lzdGVtL2NvbnRyb2wtc3RhdGlvblwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL2hvbWUvdnJ1c2hhbmdhbmFuZC9EZXNrdG9wL2h4OS1jb250cm9sLXN5c3RlbS9jb250cm9sLXN0YXRpb24vdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL2hvbWUvdnJ1c2hhbmdhbmFuZC9EZXNrdG9wL2h4OS1jb250cm9sLXN5c3RlbS9jb250cm9sLXN0YXRpb24vdml0ZS5jb25maWcudHNcIjtpbXBvcnQgcmVhY3QgZnJvbSBcIkB2aXRlanMvcGx1Z2luLXJlYWN0LXN3Y1wiO1xuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcblxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XG5cdHBsdWdpbnM6IFtyZWFjdCgpXSxcblx0cmVzb2x2ZToge1xuXHRcdGFsaWFzOiB7XG5cdFx0XHRcIkAvXCI6IFwiL3NyYy9cIixcblx0XHR9LFxuXHR9LFxuXHRzZXJ2ZXI6IHtcblx0XHRwcm94eToge1xuXHRcdFx0XCIvc29ja2V0LmlvXCI6IHtcblx0XHRcdFx0dGFyZ2V0OiBcIndzOi8vMTI3LjAuMC4xOjUwMDBcIixcblx0XHRcdFx0d3M6IHRydWUsXG5cdFx0XHR9LFxuXHRcdH0sXG5cdH0sXG59KTtcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBNFcsT0FBTyxXQUFXO0FBQzlYLFNBQVMsb0JBQW9CO0FBRzdCLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQzNCLFNBQVMsQ0FBQyxNQUFNLENBQUM7QUFBQSxFQUNqQixTQUFTO0FBQUEsSUFDUixPQUFPO0FBQUEsTUFDTixNQUFNO0FBQUEsSUFDUDtBQUFBLEVBQ0Q7QUFBQSxFQUNBLFFBQVE7QUFBQSxJQUNQLE9BQU87QUFBQSxNQUNOLGNBQWM7QUFBQSxRQUNiLFFBQVE7QUFBQSxRQUNSLElBQUk7QUFBQSxNQUNMO0FBQUEsSUFDRDtBQUFBLEVBQ0Q7QUFDRCxDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
