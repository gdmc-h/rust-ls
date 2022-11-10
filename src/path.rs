use colored::*;
use std::{fs, process};

use crate::file::File;

#[derive(Default)]
pub struct Path {
    pub folders: Vec<File>,
    pub files: Vec<File>,
    pub root: Vec<File>
}

impl Path {
    fn set_files<T, A>(&mut self, files: Vec<File>, filter_condition_folder: T, filter_condition_file: A)
    where T: Fn(&&File) -> bool,
          A: Fn(&&File) -> bool { // there's probably a better solution to this C-c/C-v
        self.root = files.clone();
        self.files = files.iter().filter(filter_condition_file).cloned().collect();
        self.folders = files.iter().filter(filter_condition_folder).cloned().collect();
    }

    fn sort_files(&mut self) {
        self.files.sort_by(|a, b| a.file_name.cmp(&b.file_name));
        self.folders.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    }

    fn print_files(&self) {
        let format_string = |file: &File| {
            let tab_size = self.root.clone().into_iter().fold(0, |a, b| {
                if b.file_name.len() > a {
                    b.file_name.len()
                } else {
                    a
                }
            });
            let tab = (0..tab_size - file.file_name.len()).map(|_| " ").collect::<String>();
            let print_file = |color: &str| {
                println!(
                    "{} {}\t{} {}mb", 
                    file.file_name.color(color),
                    tab,
                    file.permissions,
                    file.size
                );
            };

            if file.is_folder {
                print_file("blue");
            } else {
                print_file("white");
            }
        };


        self.folders.iter().for_each(format_string);
        self.files.iter().for_each(format_string);
    }

    fn scan_folder(&self, path: &str) -> Vec<File> {
        let scan_folder = fs::read_dir(path);
         match scan_folder {
            Ok(e) => e.map(File::fetch_file).collect(),
            Err(e) => {
                println!("{}: {}", path, e);
                process::exit(0);
            }
        }
    }

    pub fn show_path(&mut self, to_path: &str) {
        let files = self.scan_folder(to_path);
        self.set_files(files, |p| p.is_folder, |a| !a.is_folder);
        self.sort_files();
        self.print_files();
    }
}

