use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::Manager;

mod builder;
mod importer;

#[derive(Parser)]
#[command(
    name = "slideapp",
    about = "Vite + Rust(Tauri) 製 HTMLスライドアプリ (nvim運用向け)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// GUIウィンドウでプレゼンを開始 (Viteの開発サーバーを内部で起動する。デフォルト)
    Present,
    /// 静的サイトとしてビルドするだけ (`vite build` を実行。GUIは開かない)
    Build,
    /// 新しいスライドファイルを雛形付きで作成 (nvimで開く用)
    New {
        name: String,
        /// テンプレート種別: basic / title / ts / ss / cs
        #[arg(short, long, default_value = "basic")]
        template: String,
    },
    /// Slidev形式のMarkdownファイルをsrc/slides/にインポートする(本文のみ、frontmatterは無視)
    Import { path: std::path::PathBuf },
}

/// Viteの開発サーバープロセスをTauriのstateとして保持し、終了時にkillするための入れ物
struct ViteProcess(Mutex<Option<Child>>);

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Present) {
        Commands::Build => {
            run_npm(&["run", "build"])?;
            println!("dist/ にビルドしました");
        }
        Commands::New { name, template } => {
            builder::new_slide(&name, &template)?;
        }
        Commands::Import { path } => {
            importer::import(&path)?;
        }
        Commands::Present => run_gui()?,
    }
    Ok(())
}

/// Windowsでは `npm` の実体が `npm.cmd` のため、Command::new("npm") だけでは
/// "program not found" になる。OSに応じて呼び分ける。
fn npm_command() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("npm.cmd")
    } else {
        Command::new("npm")
    }
}

fn run_npm(args: &[&str]) -> Result<()> {
    let status = npm_command()
        .args(args)
        .status()
        .map_err(|e| anyhow!("npmの実行に失敗しました: {e} (プロジェクトルートで実行していますか?)"))?;
    if !status.success() {
        bail!("npmがエラー終了しました");
    }
    Ok(())
}

fn run_gui() -> Result<()> {
    // NVIDIAドライバ環境でWebKitGTKのDMA-BUFレンダラーが
    // "Failed to create GBM buffer" エラーを起こす既知の問題への対策。
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    println!("Viteの開発サーバーを起動しています...");
    let vite_child = npm_command()
        .args(["run", "dev"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("`npm run dev` の起動に失敗しました。プロジェクトルートで `npm install` は実行済みですか?")?;

    // Viteがポートを開くまで少し待つ
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let app = tauri::Builder::default()
        .manage(ViteProcess(Mutex::new(Some(vite_child))))
        .build(tauri::generate_context!())
        .expect("tauriアプリの構築に失敗しました");

    // ウィンドウが閉じられたら、内部で起動したVite開発サーバーも一緒に終了させる
    app.run(|app_handle, event| {
        if matches!(
            event,
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit
        ) {
            if let Some(state) = app_handle.try_state::<ViteProcess>() {
                if let Ok(mut guard) = state.0.lock() {
                    if let Some(mut child) = guard.take() {
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                }
            }
        }
    });

    Ok(())
}
