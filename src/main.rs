use std::path::Path;
use serde_yml;

use crate::source::SourceLoader;

mod source;

fn main() {
    let loader: SourceLoader = SourceLoader::default(
        Path::new("src/source")
    );
}
