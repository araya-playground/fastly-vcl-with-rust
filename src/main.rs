use std::fs::File;

use handlebars::{to_json, Handlebars};
use serde_json::Map;

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("vcl_main", "src/vcl/main.vcl.hbs")
        .unwrap();

    let mut data = Map::new();
    data.insert("debugText".to_string(), to_json("hello world"));

    let mut output_file = File::create("dist/main.vcl").unwrap();

    handlebars
        .render_to_write("vcl_main", &data, &mut output_file)
        .unwrap();
}
