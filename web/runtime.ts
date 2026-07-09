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
  if (!app) return;

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

/** entry.generated.ts から呼ばれるエントリーポイント */
export function mountSlides(defs: SlideDef[]) {
  slides = defs;
  current = 0;
  render();

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
