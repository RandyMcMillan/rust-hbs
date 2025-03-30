use std::fs;
use std::path::Path;

use handlebars::*;
use serde::Serialize;
use once_cell::sync::OnceCell;

#[derive(Serialize)]
struct Context {
    name: String,
    version: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template_content_lib = include_str!("lib_template.hbs");
    let template_content_main = include_str!("main_template.hbs");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path_lib = Path::new(&out_dir).join("lib.rs");
    let out_path_main = Path::new(&out_dir).join("main.rs");

    let name = std::env::var("CARGO_PKG_NAME").unwrap_or("default_name".to_string());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or("0.1.0".to_string());

    let reg = Handlebars::new();
    let data = Context {
        name: name,
        version: version,
    };

    let rendered_lib = reg.render_template(template_content_lib, &data)?;
    let rendered_main = reg.render_template(template_content_main, &data)?;

    fs::write(out_path_lib, rendered_lib)?;
    fs::write(out_path_main, rendered_main)?;

    Ok(())
}
