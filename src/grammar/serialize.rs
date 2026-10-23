// Serialization; only used by Grammar<Count>

use std::{fs, io, path::Path};

use crate::grammar::{Count, Grammar};

/// Only de/serialize model in the more space-efficient mode
impl Grammar<Count> {
    /// Load model from file at path
    pub fn load(path: &Path) -> io::Result<Grammar<Count>> {
        postcard::from_bytes(&fs::read(path)?).map_err(io::Error::other)
    }

    /// Save model to file at path
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let inner = fs::File::create(path)?;
        let writer = io::BufWriter::new(inner);
        postcard::to_io(self, writer).map_err(io::Error::other)?;
        Ok(())
    }
}
