import { slide } from "../runtime";

export default slide({
  title: "CLI(コマンドライン)",
  content: `
    <h1>CLI(コマンドライン)</h1>
    <ul>
      <li>GUIを持たず、ターミナルだけで操作する</li>
      <li>軽量でリソース消費が少ない</li>
      <li>SSH経由の遠隔操作・サーバー用途と相性が良い</li>
      <li>代表的なディストリ: Arch Linux(最小構成), Debian netinst, Ubuntu Server, Alpine Linux</li>
    </ul>
  `,
});
