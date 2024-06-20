use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum SourceFormat {
    YAML
}

#[derive(Debug, PartialOrd, PartialEq)]
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

#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub(crate) struct SourceCacheKey(String);

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

    pub(crate) fn source_type(&self) -> &SourceType {
        return &self.source_type
    }

    pub(crate) fn path(&self) -> &std::path::Path {
        return self.path_buf.as_path()
    }

    pub(crate) fn cache_key(&self) -> SourceCacheKey {
        return SourceCacheKey(self.name.clone())
    }
}
