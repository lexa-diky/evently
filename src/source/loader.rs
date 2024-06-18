use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use pathdiff;

use walkdir::DirEntry;

use crate::source::{SourceFormat, SourceType};
use crate::source::source_ref::SourceReference;

#[derive(Debug)]
pub(crate) enum Error {
    IO { path: Option<PathBuf> },
    UnsupportedExtension { extension: String, source_type: SourceType },
    Unexpected(String),
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
                    if let Some(reference) = Self::load_file(&dir, &self.root_path)? {
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

    fn load_file(dir_entry: &DirEntry, root_path: &PathBuf) -> Result<Option<SourceReference>, Error> {
        if !dir_entry.file_type().is_file() {
            return Ok(None);
        }

        let path = dir_entry.path();
        let mut source_type: SourceType = SourceType::DECLARATION;

        let file_stem_optional = path.file_stem().and_then(|f| f.to_str());
        let file_stem = file_stem_optional.ok_or(
            Error::Unexpected(String::from("File stem is not a valid UTF-8 string"))
        )?;

        if file_stem == Self::MANIFEST_FILE_NAME {
            source_type = SourceType::MANIFEST
        }

        let is_yaml = path.extension() == Some(OsStr::new(Self::YAML_EXTENSION));
        let is_yml = path.extension() == Some(OsStr::new(Self::YML_EXTENSION));

        if !(is_yaml | is_yml) {
            return Err(
                Error::UnsupportedExtension {
                    extension: path.extension().unwrap().to_str().unwrap().to_string(),
                    source_type: source_type,
                }
            );
        }

        Ok(
            Some(
                SourceReference::new(
                    dir_entry.path().to_path_buf(),
                    Self::collocate_rel_path(path, root_path)?,
                    file_stem.to_string(),
                    SourceFormat::YAML,
                    source_type,
                )
            )
        )
    }

    fn collocate_rel_path(source: &Path, root: &PathBuf) -> Result<Vec<String>, Error> {
        let source_file_name = source.file_name()
            .map(|it| it.to_string_lossy())
            .ok_or_else(|| Error::Unexpected(
                format!("Unexpected source file name: {:?}", source)
            ))?;

        let diff = pathdiff::diff_paths(source, root)
            .ok_or_else(||
            Error::Unexpected(
                format!(
                    "Failed to calculate relative path\nsource={:?}\nroot={:?}",
                    source,
                    root
                )
            ))?;

        let mut buf = Vec::new();
        for component in diff.components() {
            match component {
                Component::Normal(name) => {
                    let name_string = name.to_string_lossy().to_string();
                    if source_file_name != name_string {
                        buf.push(name_string)
                    }

                }
                _ => return Err(
                    Error::Unexpected(
                        format!("Unexpected path component: {:?}", component)
                    )
                )
            }
        }

        return Ok(buf);
    }
}