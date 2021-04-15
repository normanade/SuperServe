//! generates templates of static HTML files with `Handlebars`

use std::fs::{create_dir, File, copy};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use handlebars::Handlebars;

use crate::config::get_config;

/// render each template and saves it to `TEMPLATE_DIR`
fn engine_write_templates(templates: serde_json::map::Iter) {
    let config = get_config();

    let static_dir = config["static_directory"].to_string().replace("\"", "");

    let template_variables = config["template_variables"].as_object().unwrap();

    // iterates through all the static files and renders it
    for (_key, value) in templates {
        let filename = value.to_string().replace("\"", "");

        let template_file_path = format!("{}/{}", static_dir, filename);
        let template_file = File::open(template_file_path.to_string()).unwrap();
        let mut buf_reader = BufReader::new(template_file);
        let mut template_file_content = String::new();

        let template_dir = config["template_directory"].to_string().replace("\"", "");
        let file_to_write = format!("{}/{}", template_dir, filename);
        match buf_reader.read_to_string(&mut template_file_content) {
            // Document is in UTF-8 then render with handlebars
            Ok(_) => {
                // render the template with the `template_variables` from `serve_opt.json`
                let rendered_template = Handlebars::new()
                    .render_template(&template_file_content, &serde_json::json!(template_variables))
                    .unwrap();
                let mut file = File::create(file_to_write).unwrap();
                file.write_all(rendered_template.as_bytes()).unwrap();
            },
            // Document not in UTF-8 then just copy & paste
            Err(_) => {
                copy(template_file_path, file_to_write).unwrap();
            }
        };
    }
}

/// pass all the static HTML files to validate and render
pub fn render_templates() {
    let config = get_config();
    let template_dir = config["template_directory"].to_string().replace("\"", "");

    // create templates directory if it doesn't exist
    if !Path::new(&template_dir).exists() {
        create_dir(template_dir).unwrap();
    }

    // converting JSON structures to iterable structures
    let templates = config["routes"].as_object().unwrap().into_iter();
    engine_write_templates(templates);
}
