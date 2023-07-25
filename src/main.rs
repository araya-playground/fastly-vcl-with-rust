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

