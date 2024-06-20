use crate::model::Manifest;
use crate::source::{SourceReader, SourceReference};

#[derive(Debug)]
pub(crate) enum Error {

}

pub(crate) struct ManifestParser {
    source_reader: SourceReader,
}

impl ManifestParser {

    pub(crate) fn new(source_reader: SourceReader) -> ManifestParser {
        ManifestParser {
            source_reader: source_reader,
        }
    }

    pub(crate) fn parse(&self, source_ref: &SourceReference) -> Result<Manifest, Error> {
        let source_text = self.source_reader.read(source_ref).unwrap();
        unimplemented!("Parse manifest from source text")
    }
}