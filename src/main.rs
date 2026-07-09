use clap::{Parser, Subcommand};

mod builder;
mod watcher;

#[derive(Parser)]
#[command(
    name = "slideapp",
    about = "TypeScript + CSS 製 HTMLスライドアプリ (GUI + nvim運用)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// GUIウィンドウでプレゼンを開始し、保存を検知して自動リロード (デフォルト)
    Present,
    /// dist/ に静的ビルドするだけ (GUIは開かない)
    Build,
    /// 新しいスライドファイルを雛形付きで作成 (nvimで開く用)
    New { name: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Present) {
        Commands::Build => {
            builder::build()?;
            println!("dist/ にビルドしました");
        }
        Commands::New { name } => {
            builder::new_slide(&name)?;
        }
        Commands::Present => {
            run_gui()?;
        }
    }
    Ok(())
}

fn run_gui() -> anyhow::Result<()> {
    // 初回ビルド
    builder::build()?;

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            watcher::watch(handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("tauriアプリの起動に失敗しました");

    Ok(())
}
