use std::fs::{self, metadata};
use std::path::Path;

fn main() {
    // check if folder named bookmarks exists.
    //
    let dirs;
    match fs::read_dir("./bookmarks/") {
        Ok(_) => {
            println!("bookmarks dir found.");
            dirs = fs::read_dir("./bookmarks").unwrap();
        }
        Err(_) => {
            fs::create_dir("./bookmarks/").unwrap();
            dirs = fs::read_dir("./bookmarks").unwrap();
        }
    }

    for dir in dirs {
        println!("{:?}", metadata(dir.unwrap().path()));
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
