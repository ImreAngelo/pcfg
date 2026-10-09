use crate::grammar::model::Model;

pub mod occurrences;
mod classify;
mod process;

pub fn train(path: &str) -> Result<Model, std::io::Error> {
    let data = process::process_file(path)?;
    Ok(Model::from(data))
}
