import { defineConfig } from 'vite'
import tailwindcss from "@tailwindcss/vite"
import path from "path"
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://10.13.21.35:1001",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "/api"),
      },
    },
  },
})
