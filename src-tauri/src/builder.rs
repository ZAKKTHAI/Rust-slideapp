use anyhow::{Context, Result};
use std::fs;

/// nvimで開く用の新規スライドファイルを雛形付きで作成する。
/// プロジェクトルート(package.jsonがある場所)で実行される想定。
pub fn new_slide(name: &str, template: &str) -> Result<()> {
    // 誤って ".ts" 付きの名前を渡されても二重拡張子にならないようにする
    let name = name.strip_suffix(".ts").unwrap_or(name);

    fs::create_dir_all("src/slides")
        .context("src/slides/ を作成できません(プロジェクトルートで実行していますか?)")?;
    let count = fs::read_dir("src/slides")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "ts").unwrap_or(false))
        .count();

    let filename = format!("src/slides/{:02}-{}.ts", count + 1, name);
    let body = render_template(name, template);
    fs::write(&filename, body)?;
    println!("作成しました: {filename}  (nvim {filename} で編集を開始できます)");
    Ok(())
}

/// テンプレート種別ごとのTSファイル本文を生成する
///
/// - basic: 見出しだけのまっさらな雛形
/// - title: 表紙・タイトルスライド用
/// - ts:    TS(Topic Sentence)だけを書く出発点
/// - ss:    TS + SS(根拠)まで書いた状態
/// - cs:    TS + SS + CS(結論の言い換え)まで揃ったフルセット
fn render_template(name: &str, template: &str) -> String {
    let content = match template {
        "title" => format!(
            "    <h1>{name}</h1>\n    <p>サブタイトル・発表者名・日付など</p>\n"
        ),
        "ts" => format!(
            "    <h1>{name}</h1>\n    <p>\n      <!-- TS: このパラグラフの主張(結論)を一文で -->\n      ここに主張(結論)を書く。\n    </p>\n"
        ),
        "ss" => format!(
            "    <h1>{name}</h1>\n    <p>\n      <!-- TS -->\n      ここに主張(結論)を書く。\n      <!-- SS1 -->\n      根拠1を書く。\n      <!-- SS2 (根拠が複数あればここに追加) -->\n      根拠2を書く。\n    </p>\n"
        ),
        "cs" => format!(
            "    <h1>{name}</h1>\n    <p>\n      <!-- TS: 主張を一文で -->\n      ここに主張(結論)を書く。\n      <!-- SS: 根拠(複数可) -->\n      根拠1を書く。\n      根拠2を書く。\n      <!-- CS: TSを結論として別表現で言い換え -->\n      したがって、○○という結論になる。\n    </p>\n"
        ),
        _ => format!("    <h1>{name}</h1>\n"),
    };

    format!(
        "import {{ slide }} from \"../runtime\";\n\nexport default slide({{\n  title: \"{name}\",\n  content: `\n{content}  `,\n}});\n"
    )
}
