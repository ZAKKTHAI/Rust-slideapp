import { slide } from "../runtime";

export default slide({
  title: "はじめに",
  content: `
    <h1>Rust + Vite製 スライドアプリ</h1>
    <p>Viteの開発サーバーがHMRを担当し、Rust(Tauri)はGUIウィンドウとCLIを担当する</p>
  `,
});
