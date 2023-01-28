use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(RustEmbed)]
#[folder = "src/workdir/templates"]
struct Templates;

pub fn create_file_with_template(
    filepath: &str,
    template_name: &str,
    template_data: &impl Serialize,
) -> Result<(), Error> {
    let template = Templates::get(template_name);
    if template.is_none() {
        return Err(Error {
            kind: ErrorKind::TemplateNotFound,
            message: format!("template with name {} not found", template_name),
        });
    }

    let binding = String::from_utf8(template.unwrap().data.to_vec()).unwrap();
    let content_text = binding.as_str();

    let reg = Handlebars::new();
    let result = reg.render_template(content_text, template_data);
    if result.is_err() {
        return Err(Error {
            kind: ErrorKind::RenderTemplateNotFound,
            message: format!(
                "render template {} err: {}",
                template_name,
                result.err().unwrap()
            ),
        });
    }

    let file_content = result.unwrap();

    // create settings.yaml
    let mut file = match File::create(filepath) {
        Err(err) => {
            return Err(Error {
                kind: ErrorKind::CreateFileError,
                message: format!("create file {} err: {}", filepath, err.to_string()),
            })
        }
        Ok(file) => file,
    };

    match file.write_all(file_content.as_bytes()) {
        Err(why) => {
            return Err(Error {
                kind: ErrorKind::WriteFileError,
                message: format!("write to file {} err: {}", filepath, why),
            })
        }
        Ok(_) => (),
    }

    Ok(())
}
