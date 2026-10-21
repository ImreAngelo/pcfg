use crate::grammar::model::Model;

// mod v1;
mod v2;

// pub enum Version {
//     V1,
//     V2
// }

pub fn generate(model: Model) -> Box<dyn Iterator<Item = String>> {
    // match version {
    //     Version::V1 => Box::new(v1::Generator::new(model)),
    //     Version::V2 => Box::new(v2::Generator::new(model)),
    // }
    Box::new(v2::Generator::new(model))
}