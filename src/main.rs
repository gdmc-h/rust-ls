/*
 * ls' millennial cousin.
 * program made in order to learn how rust works.
 * warning: it probably contains nonsense to a senior Rust developer; the transition from 4 years of coding EXCLUSIVELLY in Typescript to Rust is HARD.
 *
 * - Giuseppe D'Amico
 */

extern crate colored;

mod file;

use file::{File, Path};
use std::{fs, env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("./");
    let to_folder: &String = match args.get(1) { 
        Some(e) => e,
        None => &default_path
    };

    let scan_folder = fs::read_dir(&to_folder);

    let root: Vec<File> = match scan_folder {
        Ok(e) => {
            e
            .map(|p| {
                let res = p.unwrap();
                let file_name = String::from(res.file_name().to_str().unwrap());
                let is_folder = res.file_type().unwrap().is_dir();

                File {
                    file_name,
                    is_folder
                }
            }).collect()
        },
        Err(_) => {
            println!("Path {} does not exist", &to_folder);
            process::exit(0);
        }
    };

    let mut path = Path {..Default::default()};
    path.show_path(&root);
}
