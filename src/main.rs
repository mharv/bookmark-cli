use serde_json::{json, Value};
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
    
    // this file should be named something fixed
    match File::open("./bookmarks-2023-03-25.json") {
        Ok(mut file) => {
            file.read_to_string(&mut imported_file_contents).unwrap();
        }
        Err(_) => {
            println!("You require the file \"ssot.json\" to exist");
        }
    }

    // pretty print
    let imported_file_contents_json: Value = serde_json::from_str(&imported_file_contents).unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&imported_file_contents_json).unwrap()
    );

    // test some encryption/decryption here



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
