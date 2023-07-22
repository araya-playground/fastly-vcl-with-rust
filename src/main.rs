use std::{error::Error, fs::File, io::BufReader};

use handlebars::{to_json, Handlebars};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JSONValue};

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("vcl_main", "src/vcl/main.vcl.hbs")
        .unwrap();

    handlebars
        .register_template_file("vcl_backends", "src/vcl/backends.vcl.hbs")
        .unwrap();

    let data = build_handlebars_data();

    let mut output_file = File::create("dist/main.vcl").unwrap();

    handlebars
        .render_to_write("vcl_main", &data, &mut output_file)
        .unwrap();

    handlebars
        .render_to_write(
            "vcl_backends",
            &data,
            &mut File::create("dist/backends.vcl").unwrap(),
        )
        .unwrap();
}

fn build_handlebars_data() -> Map<String, JSONValue> {
    let mut data = Map::new();
    let backends = build_backends().unwrap();
    data.insert("debugText".to_string(), to_json("hello world"));
    data.insert("backends".to_string(), to_json(backends));

    data
}

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

fn build_backends() -> Result<Vec<Backend>, Box<dyn Error>> {
    let file = File::open("src/backends.json")?;
    let reader = BufReader::new(file);
    let data: Vec<Backend> = serde_json::from_reader(reader)?;
    Ok(data)
}
