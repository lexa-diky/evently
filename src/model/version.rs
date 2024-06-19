use regex::Regex;

pub(crate) enum Version {
    Latest,
    Semver { major: u32, minor: u32, patch: u32 },
    Custom { value: String },
}

impl Version {
    const LATEST_VALUE: &'static str = "latest";
    const VERSION_SEPARATOR: &'static str = ".";
    const SEMVER_REGEX: &'static str = r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)";
    const SEMVER_REGEX_KEY_MAJOR: &'static str = "major";
    const SEMVER_REGEX_KEY_MINOR: &'static str = "minor";
    const SEMVER_REGEX_KEY_PATCH: &'static str = "patch";

    pub(crate) fn parse(text: &str) -> Option<Version> {
        return Self::try_parse(text)
            .or_else(|| Some(Version::Custom { value: text.to_string() }));
    }

    fn try_parse(text: &str) -> Option<Version> {
        if text == Self::LATEST_VALUE {
            return Some(Version::Latest);
        }

        let regex = Regex::new(Self::SEMVER_REGEX).ok()?;
        let captures = regex.captures(text)?;

        let major = Self::extract_version_group(
            Self::SEMVER_REGEX_KEY_MAJOR,
            &captures,
        )?;

        let minor = Self::extract_version_group(
            Self::SEMVER_REGEX_KEY_MAJOR,
            &captures,
        )?;

        let patch = Self::extract_version_group(
            Self::SEMVER_REGEX_KEY_MAJOR,
            &captures,
        )?;

        return Some(
            Version::Semver {
                major,
                minor,
                patch,
            }
        )
    }

    fn extract_version_group(name: &'static str, captures: &regex::Captures) -> Option<u32> {
        captures.name(name)?.as_str().parse().ok()
    }
}