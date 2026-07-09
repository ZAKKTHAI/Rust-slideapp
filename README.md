<<<<<<< HEAD
# Rust-slideapp
=======
# slideapp

Rust(Tauri) 製の GUI スライドアプリ。スライドの内容は TypeScript + CSS で書き、
nvim で編集 → 保存すると自動でビルドされ、GUI ウィンドウが自動リロードされる。

## 事前準備

1. Rust (`rustup`) をインストール
2. `esbuild` をインストールし、PATH に通す
   ```sh
   npm install -g esbuild
   # または: curl -fsSL https://esbuild.github.io/dl/latest | sh
   ```
3. Tauri のビルドに必要なシステム依存関係を入れる
   (OS ごとに異なるため公式ドキュメント参照)
   https://v2.tauri.app/start/prerequisites/

   Linux (Debian/Ubuntu系) の例:
   ```sh
   sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
     libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
   ```

## 使い方

```sh
# 依存クレートを取得してGUIを起動 (デフォルトコマンド)
cargo run -- present

# スライドを追加 (nvimで編集する用のファイルを雛形付きで作成)
cargo run -- new "第1章"
# => slides/03-第1章.ts が作成される。nvim slides/03-第1章.ts で編集開始

# 静的ファイルとしてビルドするだけ (GUIは開かない。共有・配布用)
cargo run -- build
# => dist/index.html, dist/bundle.js, dist/bundle.css が生成される
```

## nvim 運用イメージ

1. ターミナルの1ペインで `cargo run -- present` を起動しっぱなしにする (GUIウィンドウが開く)
2. 別ペインで nvim を開き `slides/*.ts` や `web/styles/theme.css` を編集
3. `:w` で保存するたびに自動で再ビルドされ、GUI ウィンドウが自動でリロードされる
4. 新しいスライドが欲しくなったら `cargo run -- new "タイトル"` → 出力されたパスを nvim で開く

## スライドの書き方

`slides/` 配下に `NN-name.ts` という形式でファイルを置くと、番号順に読み込まれる。

```ts
import { slide } from "../web/runtime";

export default slide({
  title: "タイトル",
  content: `
    <h1>見出し</h1>
    <p>本文</p>
  `,
});
```

- キー操作: `→` / `Space` / `PageDown` で次へ、`←` / `PageUp` で前へ、`Home`/`End` で最初/最後へ
- クリック: 画面右半分クリックで次へ、左半分で前へ

## 補足・既知の注意点

- 本リポジトリの内容はネットワーク接続のないサンドボックス内で作成したため、
  `cargo build` によるクレート取得や実機での動作確認は行えていません。
  お手元でビルドする際、Tauri のバージョン差異等で `tauri.conf.json` の
  スキーマや API に軽微な修正が必要になる場合があります。
- 配布用にアプリをバンドルする場合は `tauri.conf.json` の `bundle.active` を
  `true` にし、アイコンなどを追加設定してください。
>>>>>>> bf0363c (first)
