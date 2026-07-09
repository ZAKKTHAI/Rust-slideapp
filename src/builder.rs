use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

/// slides/ と web/ の内容から dist/ を生成する
pub fn build() -> Result<()> {
    fs::create_dir_all("dist")?;
    generate_entry()?;

    // TypeScript を esbuild でバンドル (ESモジュール出力)
    run_esbuild(&[
        "web/entry.generated.ts",
        "--bundle",
        "--outfile=dist/bundle.js",
        "--format=esm",
        "--target=es2020",
    ])?;

    // CSS も esbuild でバンドル
    run_esbuild(&[
        "web/styles/theme.css",
        "--bundle",
        "--outfile=dist/bundle.css",
    ])?;

    fs::copy("web/index.html", "dist/index.html")
        .context("web/index.html が見つかりません")?;

    Ok(())
}

/// slides/*.ts を番号順に読み込み、まとめてimportするentryファイルを自動生成する
fn generate_entry() -> Result<()> {
    let mut slide_files: Vec<_> = fs::read_dir("slides")
        .context("slides/ ディレクトリが見つかりません")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "ts").unwrap_or(false))
        .collect();
    slide_files.sort();

    let mut entry = String::from("import { mountSlides } from \"./runtime\";\n\n");
    let mut names = Vec::new();

    for (i, path) in slide_files.iter().enumerate() {
        let name = format!("slide{i}");
        let stem = path.file_stem().unwrap().to_string_lossy();
        entry.push_str(&format!("import {name} from \"../slides/{stem}\";\n"));
        names.push(name);
    }

    entry.push_str(&format!("\nmountSlides([{}]);\n", names.join(", ")));
    fs::write("web/entry.generated.ts", entry)?;
    Ok(())
}

fn run_esbuild(args: &[&str]) -> Result<()> {
    let status = Command::new("esbuild")
        .args(args)
        .status()
        .context("esbuildの実行に失敗しました。`npm i -g esbuild` 等で導入してください")?;
    if !status.success() {
        anyhow::bail!("esbuildがエラー終了しました");
    }
    Ok(())
}

/// nvimで開く用の新規スライドファイルを雛形付きで作成する
pub fn new_slide(name: &str) -> Result<()> {
    fs::create_dir_all("slides")?;
    let count = fs::read_dir("slides")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "ts").unwrap_or(false))
        .count();

    let filename = format!("slides/{:02}-{}.ts", count + 1, name);
    let template = format!(
        "import {{ slide }} from \"../web/runtime\";\n\nexport default slide({{\n  title: \"{name}\",\n  content: `\n    <h1>{name}</h1>\n  `,\n}});\n"
    );
    fs::write(&filename, template)?;
    println!("作成しました: {filename}  (nvim {filename} で編集を開始できます)");
    Ok(())
}
