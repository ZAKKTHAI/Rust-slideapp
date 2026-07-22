import { slide } from "../runtime";

export default slide({
  title: "使い方",
  content: `
    <h1>nvimでの運用</h1>
    <ul>
      <li>npm run present でGUIを起動</li>
      <li>nvimで src/slides/*.ts を編集して :w</li>
      <li>Viteが自動検知してホットリロード</li>
      <li>npm run new -- "タイトル" で新規スライドを追加</li>
    </ul>
  `,
});
