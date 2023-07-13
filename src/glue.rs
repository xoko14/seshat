use seed::prelude::{js_sys, web_sys};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::sleep;

#[wasm_bindgen(inline_js = "export function invoke(cmd, args) { console.log(args); return window.__TAURI_INVOKE__(cmd, args) }")]
extern "C"{
    fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct EmptyStruct;

pub async fn run_server(){
    _ = invoke("run_server", to_value(&EmptyStruct{}).unwrap());
    sleep(500).await;
}