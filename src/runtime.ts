export interface SlideDef {
  title?: string;
  content: string;
}

/** slides/*.ts から呼び出すヘルパー。型付けのためだけに存在する */
export function slide(def: SlideDef): SlideDef {
  return def;
}

let slides: SlideDef[] = [];
let current = 0;

function render() {
  const app = document.getElementById("app");
  if (!app || slides.length === 0) return;

  app.innerHTML = `
    <section class="slide">
      ${slides[current].content}
    </section>
    <div class="progress">${current + 1} / ${slides.length}</div>
  `;
}

function next() {
  if (current < slides.length - 1) {
    current++;
    render();
  }
}

function prev() {
  if (current > 0) {
    current--;
    render();
  }
}

let bound = false;

/** main.ts から呼ばれるエントリーポイント。HMRで何度呼ばれても安全なようにしてある */
export function mountSlides(defs: SlideDef[]) {
  // 直前のスライド番号を可能な範囲で保持する (HMR時にも1枚目に戻りすぎないように)
  const prevIndex = current;
  slides = defs;
  current = Math.min(prevIndex, Math.max(slides.length - 1, 0));
  render();

  if (bound) return;
  bound = true;

  window.addEventListener("keydown", (e) => {
    if (e.key === "ArrowRight" || e.key === " " || e.key === "PageDown") next();
    if (e.key === "ArrowLeft" || e.key === "PageUp") prev();
    if (e.key === "Home") {
      current = 0;
      render();
    }
    if (e.key === "End") {
      current = slides.length - 1;
      render();
    }
  });

  window.addEventListener("click", (e) => {
    const isRightHalf = e.clientX > window.innerWidth / 2;
    if (isRightHalf) next();
    else prev();
  });
}
