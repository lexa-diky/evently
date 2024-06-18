use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum SourceFormat {
    YAML
}

#[derive(Debug)]
pub(crate) enum SourceType {
    DECLARATION, MANIFEST
}

#[derive(Debug)]
pub(crate) struct SourceReference {
    path_buf: PathBuf,
    rel_path: Vec<String>,
    name: String,
    format: SourceFormat,
    source_type: SourceType
}

impl SourceReference {

    pub(crate) fn new(
        path_buf: PathBuf,
        rel_path: Vec<String>,
        name: String,
        format: SourceFormat,
        source_type: SourceType
    ) -> SourceReference {
        SourceReference {
            path_buf: path_buf,
            rel_path: rel_path,
            name: name,
            format: format,
            source_type: source_type
        }
    }
}