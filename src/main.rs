use std::fs::{self, metadata, File};
use std::io::Read;
use std::path::Path;

fn main() {

    // check if folder named bookmarks exists.
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

    // if not, create one from json file
    // check if file exists

    let mut imported_file_contents = String::new();
    match File::open("./ssot.json") {
        Ok(mut file) => {
            file.read_to_string(&mut imported_file_contents).unwrap();
        }
        Err(_) => {
            println!("You require the file \"ssot.json\" to exist");
        }
    }

    println!("{}", imported_file_contents);

    // do a git pull from a github repo.
    // do check to see if git repo exists.
    // get credentials for git client
    //
    // do auto merge.
    // todo!("git stuff comes later!");

    // convert data into filesystem after decryption

    // have following functionaility:
    // - url 200 health check (request)
    // - display urls in a folder
    // - move and delete urls.
    // - sync back to github profile
}
