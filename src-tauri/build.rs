use std::fs;
use std::path::Path;

fn main() {
    // tauri::generate_context!() はコンパイル時に frontendDist(../dist) の実在を確認する。
    // 本来の中身は `npm run build` / Vite dev server が用意するが、
    // 初回の `cargo build` の時点ではまだ無いことがあるためプレースホルダーを置く。
    let dist = Path::new("../dist");
    if !dist.exists() {
        fs::create_dir_all(dist).expect("failed to create ../dist/");
    }
    if !dist.join("index.html").exists() {
        fs::write(
            dist.join("index.html"),
            "<!doctype html><html><body>building...</body></html>",
        )
        .expect("failed to write placeholder ../dist/index.html");
    }

    let icon_path = Path::new("icons/icon.png");
    if !icon_path.exists() {
        fs::create_dir_all("icons").expect("failed to create icons/");
        // 1x1 の透明PNG (最低限これがあればコンパイルは通る)
        const MINIMAL_PNG: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x44, 0x41, 0x54, 0x78,
            0xDA, 0x63, 0x64, 0x60, 0x18, 0x05, 0x00, 0x00, 0x03, 0x00, 0x01, 0xA1, 0x1E, 0x0B,
            0x25, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];
        fs::write(icon_path, MINIMAL_PNG).expect("failed to write placeholder icons/icon.png");
    }

    tauri_build::build();
}
