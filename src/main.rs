mod source;
mod model;

use crate::source::FsSourceLoader;

fn main() {
    let loader = FsSourceLoader::default("sample/login_form");
    let sources = loader.load().unwrap();

    for r in sources {
        println!("{:?}", r)
    }
}
