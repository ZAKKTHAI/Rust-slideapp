import "./styles/theme.css";
import { mountSlides, type SlideDef } from "./runtime";

// slides/*.ts を自動収集する。ファイルを追加/削除すると、
// Viteのdevサーバーが変化を検知して自動でリロードする。
const modules = import.meta.glob("./slides/*.ts", { eager: true }) as Record<
  string,
  { default: SlideDef }
>;

function loadSlides(): SlideDef[] {
  return Object.entries(modules)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([, mod]) => mod.default);
}

mountSlides(loadSlides());
