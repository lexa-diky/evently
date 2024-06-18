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
    format: SourceFormat,
    source_type: SourceType
}

impl SourceReference {

    pub(crate) fn new(
        path_buf: PathBuf,
        format: SourceFormat,
        source_type: SourceType
    ) -> SourceReference {
        SourceReference {
            path_buf,
            format,
            source_type
        }
    }
}