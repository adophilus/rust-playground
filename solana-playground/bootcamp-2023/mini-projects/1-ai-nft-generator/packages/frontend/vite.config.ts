import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "node:path";
import tailwindcss from "@tailwindcss/vite";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import { NodeGlobalsPolyfillPlugin } from "@esbuild-plugins/node-globals-polyfill";
import { NodeModulesPolyfillPlugin } from "@esbuild-plugins/node-modules-polyfill";
import nodePolyfills from "rollup-plugin-polyfill-node";

export default defineConfig({
  plugins: [
    // NodeGlobalsPolyfillPlugin({
    //   buffer: true,
    //   process: true,
    // }),
    // nodePolyfills({
    //   include: ["stream"],
    // }),
    // NodeModulesPolyfillPlugin(),
    tailwindcss(),
    TanStackRouterVite(),
    react(),
  ],
  build: {
    outDir: "../backend/public",
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      stream: "stream-browserify",
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:5000",
      },
    },
  },
});
