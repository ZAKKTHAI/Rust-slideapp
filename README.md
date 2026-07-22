# slideapp

スライド作成をエディタ(nvim)中心のワークフローで完結させたいという課題を解決したく、
TypeScriptでスライド内容を書くとRust(Tauri)製のGUIアプリとして表示・発表できる
システムを開発しました。技術はVite(Node)によるビルド・ホットリロード基盤と
Rust(Tauri)によるネイティブGUI表示を組み合わせ、特に自前実装のesbuild+ファイル監視で
頻発していた「保存しても表示に反映されない」問題を、実績のあるViteのHMR機構に
置き換えることで解消しました。その結果、nvimで `src/slides/*.ts` を保存するだけで
即座に画面へ反映されるようになり、TS-SS-CS構成のテンプレートからの新規作成や
Slidev形式Markdownの取り込みも含めて、エディタから離れずにスライドを作れるように
なりました。

Vite(Node) + Rust(Tauri) 製の GUI スライドアプリ。スライドの内容は
TypeScriptで書き、nvimで編集 → 保存するとViteの開発サーバーが
自動でホットリロードする。

## 事前準備

1. Node.js (v18以上推奨) をインストール
2. Rust (`rustup`) をインストール
3. Tauriのビルドに必要なシステム依存関係を入れる(OSごとに異なる)
   https://v2.tauri.app/start/prerequisites/

   Linux (Debian/Ubuntu系) の例:
   ```sh
   sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
     libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
   ```
4. プロジェクトルートで依存関係をインストール
   ```sh
   npm install
   ```

## 使い方 (すべてプロジェクトルートで実行)

```sh
# GUIを起動 (Viteの開発サーバーを内部で自動起動する)
npm run present

# スライドを追加 (nvimで編集する用のファイルを雛形付きで作成)
npm run new -- "第1章"
# => src/slides/03-第1章.ts が作成される

# テンプレート種別を指定して作成 (basic / title / ts / ss / cs)
npm run new -- "結論" --template cs

# Slidev形式のMarkdownをインポート (本文のみ取り込み、frontmatterは無視)
npm run import -- path/to/slidev-deck.md

# 静的サイトとしてビルドするだけ (GUIは開かない。共有・配布用)
npm run build
# => dist/ に出力される
```

`npm run` 経由で追加の引数を渡す場合は `--` を挟む必要があります。

## nvim 運用イメージ

1. `npm run present` を起動しっぱなしにする(GUIウィンドウが開く)
2. 別ペインでnvimを開き `src/slides/*.ts` を編集
3. `:w` で保存するたびに、Viteが変更を検知して自動でホットリロードされる
4. 新しいスライドが欲しくなったら `npm run new -- "タイトル"` →
   出力されたパスをnvimで開く

## スライドの書き方

`src/slides/` 配下に `NN-name.ts` という形式でファイルを置くと、
番号順に読み込まれる。

```ts
import { slide } from "../runtime";

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
- スライドは16:9の固定サイズに収まるよう表示され、収まりきらない分は
  スライド内でスクロールできる
- Tailwind CSS (v3) が有効になっているので、`content`内のHTMLで
  `class="flex gap-4 text-4xl"` のようにユーティリティクラスがそのまま使える
  (JSXではないので `class` 属性で書く)

## テンプレート種別 (`--template`)

| 種別    | 内容                                             |
| ------- | ------------------------------------------------ |
| `basic` | (デフォルト) 見出しだけのまっさらな雛形           |
| `title` | 表紙・タイトルスライド用 (見出し+サブタイトル)    |
| `ts`    | TS(Topic Sentence)だけ書く出発点                  |
| `ss`    | TS + SS(根拠)まで書けるようになった状態           |
| `cs`    | TS + SS + CS(結論の言い換え)まで揃ったフルセット |

`ts`/`ss`/`cs`は本文中にHTMLコメント(`<!-- TS -->`など)で
書く場所の目印が入っています。コメントは表示には出ません。

## Slidevインポートについて

`npm run import` は、SlidevのMarkdownデッキ(`---`区切りのスライド)を
本アプリの `src/slides/*.ts` 形式に変換します。対応しているのは以下のみです:

- 見出し(`#`〜`###`)、段落、箇条書き、番号付きリスト
- コードブロック、インラインコード
- 表、引用、太字/斜体、画像タグ

**対応していないもの**(取り込み時に無視されます):

- frontmatter(`theme:`, `layout:` など)
- Vueコンポーネント、`<v-click>` 等のアニメーション指定
- Slidevのテーマ・CSS
- 画像の相対パス解決(必要に応じて `src/` 配下にアセットをコピーして
  パスを直してください)

## なぜVite(Node)を使っているか

以前はRust側で独自にesbuildの呼び出し・ファイル監視・キャッシュ回避を
実装していましたが、TauriのdevUrl機構と組み合わせる際に何度も
「保存しても表示が更新されない」問題が発生しました。原因は
Tauriが `frontendDist` のみの設定だとコンパイル時点の内容を
バイナリに埋め込んでしまう挙動で、自前実装の複雑さも相まって
デバッグが難航していました。

Viteは開発サーバー・HMR・キャッシュ制御が標準で実績十分なため、
今後Reactへ移行する予定も踏まえてNode/Viteベースの構成に切り替えています。
Rust(Tauri)側は「GUIウィンドウの表示」「Vite開発サーバーの起動・終了管理」
「CLI(new/import/build)」に専念する設計です。

## 補足・既知の注意点

- 本リポジトリはネットワーク制限のあるサンドボックス内で作成しており、
  `npm install` および `cargo build` のいずれもこの環境では実行・検証
  できていません(レジストリへのアクセスが許可されていないため)。
  お手元で実行した際にバージョン差異等でエラーが出た場合は、
  エラーメッセージを教えてください。
- NVIDIAのGPUドライバ環境では、WebKitGTKのDMA-BUFレンダラーが
  `Failed to create GBM buffer` エラーを起こすことがあります。
  本アプリは起動時に自動で `WEBKIT_DISABLE_DMABUF_RENDERER=1` を設定して
  この問題を回避しています。
- 配布用にアプリをバンドルする場合は `src-tauri/tauri.conf.json` の
  `bundle.active` を `true` にし、正式なアイコン等を追加設定してください。
