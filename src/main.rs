/*
 *  RStow - A GNU stow alternative written in Rust
 *
 *  Author:  Mark Broeders 
 *  Email:   inbox@markbroeders.nl
 *  Created: 27-09-2021
 *
 *  Prerequisites:
 *  - Make sure you have a folder called ".dotfiles"
 *  - Only works on linux
 *  - This is WIP, not meant to be used for real
 *
 */

use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use dirs::home_dir;

fn main() {

    let mut dotfiles_folder: PathBuf = PathBuf::new();
    let mut home_folder: PathBuf = PathBuf::new();

    if let Some(hf) = get_home() {
        dotfiles_folder = hf.join(".dotfiles");
        home_folder = hf;
    }

    if dotfiles_folder.is_dir() {
        traverse(&dotfiles_folder.clone(), dotfiles_folder, &home_folder);
    };
}

fn get_home() -> Option<PathBuf> {
    match home_dir() {
        Some(h) => {
            return Some(PathBuf::from(h));
        },
        None => {
            panic!("ERR: could not set home directory");
            },
    };
}
fn traverse(dots: &PathBuf, dir: PathBuf, home: &PathBuf) {

    if let Ok(d) = fs::read_dir(&dir) {
        for entry in d {
            let current_item = entry.unwrap().path();
            if current_item.is_dir() {
                let folder_to_create = get_path_name(&current_item, &dots, home); 
                if !folder_to_create.exists() {
                    println!("{:?} does not exist, creating folder...", &folder_to_create);
                    if let Err(_e) = fs::create_dir(folder_to_create) {
                        panic!("Could not create folder");
                    };
                };
                traverse(&dots, current_item, home);
            } else if current_item.is_file() {
                let file_to_link = get_path_name(&current_item, &dots, home); 
                if !file_to_link.exists() {
                    println!("{:?} does not exist, will create symbolic link", &file_to_link);
                    if let Err(_e) = symlink(current_item, file_to_link) {
                        panic!("Could not create symbolic link");
                    };
                } else {
                    println!("Skipping file {:?}", &file_to_link);
                };

            } else {
                panic!("Error, could not read {:?}", &current_item);
            };
        };
    };
}

fn get_path_name(current_item: &PathBuf, dotfiles_folder: &PathBuf, home: &PathBuf) -> PathBuf {
    home.join(current_item.strip_prefix(dotfiles_folder).unwrap())
}
