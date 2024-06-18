use std::ffi::OsStr;
use std::path;
use std::path::{Path, PathBuf};

use walkdir::DirEntry;

use crate::source::{SourceFormat, SourceType};
use crate::source::source_ref::SourceReference;

#[derive(Debug)]
pub(crate) enum Error {
    IO { path: Option<PathBuf> },
    UnsupportedExtension { extension: String, source_type: SourceType },
}

pub(crate) struct FsSourceLoader {
    root_path: PathBuf,
}

impl FsSourceLoader {
    pub(crate) fn default<P: AsRef<Path>>(path: P) -> FsSourceLoader {
        FsSourceLoader {
            root_path: path.as_ref().to_path_buf()
        }
    }

    pub(crate) fn load(&self) -> Result<Vec<SourceReference>, Error> {
        let mut buff: Vec<SourceReference> = Vec::new();

        let walker = walkdir::WalkDir::new(&self.root_path);

        for entry in walker {
            match entry {
                Ok(dir) => {
                    if let Some(reference) = Self::load_file(&dir)? {
                        buff.push(reference)
                    }
                }
                Err(error) => {
                    return Err(
                        Error::IO {
                            path: error.path()
                                .map(|p| p.to_path_buf())
                        }
                    )
                }
            }
        }

        return Ok(buff);
    }

    const MANIFEST_FILE_NAME: &'static str = "manifest";
    const YAML_EXTENSION: &'static str = "yaml";
    const YML_EXTENSION: &'static str = "yml";

    fn load_file(dir_entry: &DirEntry) -> Result<Option<SourceReference>, Error> {
        if !dir_entry.file_type().is_file() {
            return Ok(None);
        }

        let path = dir_entry.path();
        let mut source_type: SourceType = SourceType::DECLARATION;

        if let Some(file_name) = path.file_stem().and_then(|f| f.to_str()) {
            if file_name == Self::MANIFEST_FILE_NAME {
                source_type = SourceType::MANIFEST
            }
        }

        let is_yaml = path.extension() == Some(OsStr::new(Self::YAML_EXTENSION));
        let is_yml = path.extension() == Some(OsStr::new(Self::YML_EXTENSION));

        if ! (is_yaml | is_yml) {
            return Err(
                Error::UnsupportedExtension {
                    extension: path.extension().unwrap().to_str().unwrap().to_string(),
                    source_type: source_type
                }
            );
        }

        Ok(
            Some(
                SourceReference::new(
                    dir_entry.path().to_path_buf(),
                    SourceFormat::YAML,
                    source_type,
                )
            )
        )
    }
}