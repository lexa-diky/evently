use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::source::{SourceCacheKey, SourceReference};

pub(crate) struct SourceReader {}

impl SourceReader {
    pub(crate) fn new() -> SourceReader {
        SourceReader {}
    }

    pub(crate) fn read(&self, source_ref: &SourceReference) -> Result<String, std::io::Error> {
        let mut file = File::open(source_ref.path())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(buf)
    }
}