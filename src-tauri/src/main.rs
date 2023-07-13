// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use server::Opt;
use tokio::sync::mpsc;

mod server;

#[tauri::command]
async fn run_server(){
  let opt = Opt{
    log_level: "debug".to_owned(),
    addr: "127.0.0.1".to_owned(),
    port: 8124,
    database: "sqlite://database.db".to_owned(),
    reset_database: false,
  };
  let (rx, mut tx) = mpsc::channel::<String>(1);
  tauri::async_runtime::spawn(async move {
    server::run_server(opt, rx).await;
  });
  let result = match tx.recv().await {
    Some(v) => v,
    None => "Err".to_owned()
  };
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
