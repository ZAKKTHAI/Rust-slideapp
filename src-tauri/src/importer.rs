use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Slidev形式のMarkdownファイル(frontmatter + `---`区切りのスライド)を読み込み、
/// src/slides/ 配下に既存フォーマットの .ts ファイル群として書き出す。
///
/// 対応しているのは本文のMarkdown(見出し・段落・箇条書き・コードブロック・表など)のみ。
/// Vueコンポーネント、クリックアニメーション、テーマ/レイアウト指定などSlidev固有の
/// 機能は無視される(frontmatterごと取り除かれる)。
pub fn import(md_path: &Path) -> Result<()> {
    let raw = fs::read_to_string(md_path)
        .with_context(|| format!("{} を読み込めません", md_path.display()))?
        .replace("\r\n", "\n");

    let body = strip_frontmatter(&raw);
    let chunks: Vec<String> = body.split("\n---\n").map(|s| s.to_string()).collect();

    fs::create_dir_all("src/slides")?;
    let mut index = fs::read_dir("src/slides")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "ts").unwrap_or(false))
        .count();

    let mut written = 0;
    for chunk in &chunks {
        let content_md = strip_frontmatter(chunk).trim();
        if content_md.is_empty() {
            continue;
        }

        let html = markdown_to_html(content_md);
        let title = extract_title(content_md);
        index += 1;
        written += 1;

        let filename = format!("src/slides/{:02}-{}.ts", index, slugify(&title));
        let escaped_title = title.replace('"', "\\\"");
        let ts = format!(
            "import {{ slide }} from \"../runtime\";\n\nexport default slide({{\n  title: \"{escaped_title}\",\n  content: `\n{html}\n  `,\n}});\n"
        );
        fs::write(&filename, &ts)
            .with_context(|| format!("{filename} の書き込みに失敗しました"))?;
        println!("作成しました: {filename}");
    }

    println!(
        "{written} 枚のスライドを {} からインポートしました",
        md_path.display()
    );
    if written == 0 {
        println!("(取り込めるスライドが見つかりませんでした。ファイルの中身を確認してください)");
    }
    Ok(())
}

/// 先頭が `---` の場合、次の `---` 行までを frontmatter として取り除く
fn strip_frontmatter(input: &str) -> &str {
    let trimmed = input.trim_start_matches('\n');
    if let Some(rest) = trimmed.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---") {
            return rest[end + 4..].trim_start_matches('\n');
        }
    }
    trimmed
}

fn markdown_to_html(md: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md, opts);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

fn extract_title(md: &str) -> String {
    for line in md.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("# ") {
            return rest.trim().to_string();
        }
    }
    "スライド".to_string()
}

fn slugify(input: &str) -> String {
    let mut s: String = input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect();
    while s.contains("--") {
        s = s.replace("--", "-");
    }
    let s = s.trim_matches('-').to_string();
    if s.is_empty() {
        "slide".to_string()
    } else {
        s
    }
}
