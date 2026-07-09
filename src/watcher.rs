use anyhow::Result;
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

/// slides/ と web/ を監視し、変更があれば再ビルドしてGUIウィンドウをリロードする
pub fn watch(handle: AppHandle) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new("slides"), RecursiveMode::Recursive)?;
    watcher.watch(Path::new("web"), RecursiveMode::Recursive)?;

    thread::spawn(move || {
        // watcherをこのスレッドで保持し続ける (dropすると監視が止まる)
        let _watcher = watcher;

        for res in rx {
            if res.is_err() {
                continue;
            }

            // エディタの保存で短時間に複数イベントが飛ぶのを軽くデバウンス
            thread::sleep(Duration::from_millis(50));

            match crate::builder::build() {
                Ok(_) => {
                    if let Some(window) = handle.get_webview_window("main") {
                        let _ = window.eval("location.reload()");
                    }
                }
                Err(e) => eprintln!("ビルドエラー: {e}"),
            }
        }
    });

    Ok(())
}
