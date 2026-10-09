use core::panic;

use crate::grammar::model::Model;

mod v1;
mod v2;

pub enum Version {
    V1,
    V2
}

pub fn generate(model: Model, version: Version) -> impl Iterator<Item = String> {
    match version {
        Version::V1 => panic!("Not implemented"),
        Version::V2 => v2::Generator::new(model)
    }
}