use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

use fst::{Map, MapBuilder, Streamer};

use crate::structures::Structure;
use crate::training::Terminals;

/// Wrap an fst error as an io::Error so callers only deal with one error type.
fn fst_err(e: fst::Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
}

/// Subdirectory name per structure; also the on-disk layout, e.g. `Letters/`.
fn dir_name(structure: Structure) -> &'static str {
    match structure {
        Structure::Letters => "Letters",
        Structure::Digits => "Digits",
        Structure::Symbols => "Symbols",
    }
}

/// File holding every word of one (structure, length), e.g. `<dir>/Letters/4.fst`.
fn bucket_path(dir: &Path, structure: Structure, length: usize) -> PathBuf {
    dir.join(dir_name(structure)).join(format!("{length}.fst"))
}

/// Save each (structure, length) bucket to its own FST file under `dir`.
pub fn save(terminals: &Terminals, dir: &Path) -> io::Result<()> {
    for ((structure, length), words) in terminals {
        let path = bucket_path(dir, *structure, *length);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // FST requires keys inserted in ascending byte order, which for
        // UTF-8 strings is just their natural sort.
        let mut sorted: Vec<(&String, &u32)> = words.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));

        let writer = BufWriter::new(File::create(&path)?);
        let mut builder = MapBuilder::new(writer).map_err(fst_err)?;
        for (word, count) in sorted {
            builder.insert(word, *count as u64).map_err(fst_err)?;
        }
        builder.finish().map_err(fst_err)?;
    }

    Ok(())
}

/// Load a single bucket back into a word -> count map.
///
/// This reads the whole FST into memory and streams it into a HashMap. For
/// large buckets you'd instead memory-map the file and query the `Map`
/// directly (see `open` below) rather than materializing every entry.
pub fn load(
    dir: &Path,
    structure: Structure,
    length: usize,
) -> io::Result<HashMap<String, u32>> {
    let map = open(dir, structure, length)?;

    let mut out = HashMap::new();
    let mut stream = map.stream();
    while let Some((key, value)) = stream.next() {
        let word = String::from_utf8(key.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        out.insert(word, value as u32);
    }

    Ok(out)
}

/// Open a bucket as an FST `Map` backed by the file's bytes, without
/// materializing entries. Look words up with `map.get(word)` (returns the
/// count), or range-scan with `map.stream()`.
pub fn open(dir: &Path, structure: Structure, length: usize) -> io::Result<Map<Vec<u8>>> {
    let path = bucket_path(dir, structure, length);
    let bytes = fs::read(&path)?;
    Map::new(bytes).map_err(fst_err)
}
