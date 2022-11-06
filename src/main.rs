/*
 * ls' millennial cousin.
 * program made in order to learn how rust works.
 * warning: it probably contains nonsense to a senior Rust developer; the transition from 4 years of coding EXCLUSIVELLY in Typescript to Rust is HARD.
 *
 * - Giuseppe D'Amico
 */

extern crate colored;

mod file;

use file::{File, get_files, sort_files, print_files};
use std::{fs, rc::Rc, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("./");
    let to_folder = args.get(1).unwrap_or(&default_path);

    let root: Rc<Vec<File>> = Rc::new(
        // TODO: read_dir should get path from argv
        fs::read_dir(&to_folder).unwrap().map(|p| {
            let res = p.unwrap();
            let file_name = String::from(res.file_name().to_str().unwrap());
            let is_folder = res.file_type().unwrap().is_dir();

            File {
                file_name,
                is_folder
            }
        }).into_iter().collect()
    );

    // should probably be refactored to be more flexible
    let folders: Vec<File> = get_files(root.clone(), |p| p.is_folder);
    let files: Vec<File> = get_files(root, |p| !p.is_folder);

    let sorted_folders = sort_files(folders);
    let sorted_files = sort_files(files);

    print_files(sorted_folders);
    print_files(sorted_files);
}
