import { defineConfig } from "vite";

// Tauri用の固定ポート。src-tauri/tauri.conf.json の devUrl と必ず一致させる。
const port = 1420;

export default defineConfig({
  server: {
    port,
    strictPort: true,
    // nvim等の外部エディタでの保存を確実に検知するためポーリングも有効にする
    watch: {
      usePolling: true,
      interval: 100,
    },
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
  // Tauriのコンソールエラーを見やすくする
  clearScreen: false,
});
