import { slide } from "../web/runtime";

export default slide({
  title: "使い方",
  content: `
    <h1>nvimでの運用</h1>
    <ul>
      <li>slideapp present でGUIを起動</li>
      <li>nvimで slides/*.ts を編集して :w</li>
      <li>自動で再ビルド → ウィンドウが自動リロード</li>
      <li>slideapp new "タイトル" で新規スライドを追加</li>
    </ul>
  `,
});
