use crate::model::version::Version;
use crate::source::SourceReference;

pub(crate) struct Manifest {
    pub(crate) reference: SourceReference,

    pub(crate) name: String,
    pub(crate) dependencies: Vec<Dependency>,

}

pub(crate) struct Dependency {
    group: String,
    id: String,
    version: Version
}

impl Dependency {

    pub(crate) fn new(group: String, id: String, version: Version) -> Dependency {
        Dependency {
            group: group,
            id: id,
            version: version
        }
    }

    const DEPENDENCY_NOTATION_SEPARATOR: &'static str = ":";
    pub(crate) fn parse(dependency: &str) -> Option<Dependency> {
        let parts: Vec<&str> = dependency.split(Self::DEPENDENCY_NOTATION_SEPARATOR).collect();
        if parts.len() != 3 {
            return None
        }

        Some(
            Dependency {
                group: parts[0].to_string(),
                id: parts[1].to_string(),
                version: Version::parse(parts[2].to_string().as_str())?
            }
        )
    }
}