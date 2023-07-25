use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Deserialize, Serialize)]
struct Healthcheck {
    method: String,
    path: String,
}

#[derive(Deserialize, Serialize)]
struct Backend {
    share_key: String,
    host: String,
    port: u16,
    ssl: bool,
    ssl_cert_hostname: String,
    ssl_check_cert: String,
    ssl_sni_hostname: String,
    connect_timeout: String,
    between_bytes_timeout: String,
    first_byte_timeout: String,
    max_connections: u32,
    host_header: String,
    always_use_host_header: bool,
    healthcheck: Healthcheck,
}

#[wasm_bindgen]
pub fn build_backends(backends_json_str: &str) -> Result<JsValue, JsValue> {
    let data: Vec<Backend> = serde_json::from_str(backends_json_str).unwrap();
    Ok(serde_wasm_bindgen::to_value(&data)?)
}
