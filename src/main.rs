use std::fs::{self, DirEntry, metadata};
use std::io;
use std::path::Path;

fn main() {
    // check if folder named bookmarks exists.
    //
    let dirs = fs::read_dir(".").unwrap();

    for dir in dirs {
        if metadata(dir.as_ref().unwrap().path()).unwrap().is_dir() {
            println!("{:?}", dir);
        }
    }

    // if not, create one.

    // do a git pull from a github repo.

    // do auto merge.

    // convert data into filesystem after decryption

    // have following functionaility:
    // - url 200 health check (request)
    // - display urls in a folder
    // - move and delete urls.
    // - sync back to github profile
}
