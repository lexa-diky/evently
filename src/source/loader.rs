use std::path::{Path, PathBuf};

pub(crate) struct SourceLoader {
    root_path: PathBuf,
}

impl SourceLoader {

    pub(crate) fn default(path: &Path) -> SourceLoader {
        SourceLoader {
            root_path: path.to_path_buf()
        }
    }
}