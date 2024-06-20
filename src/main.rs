mod source;
mod model;
mod parser;

use crate::source::{FsSourceLoader, SourceReader, SourceType};

fn main() {
    let loader = FsSourceLoader::default("sample/login_form");
    let sources = loader.load().unwrap();
    let manifest_ref = sources.iter()
        .find(|source| *source.source_type() == SourceType::MANIFEST)
        .unwrap();

    let reader = SourceReader::new();
    let manifest_parser = parser::manifest::ManifestParser::new(reader);
    let source_text = manifest_parser.parse(manifest_ref).unwrap();
}
