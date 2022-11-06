use colored::*;
use std::rc::Rc;

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Clone)]
pub struct File {
    pub file_name: String,
    pub is_folder: bool
}

// sorting by name
pub fn sort_files(mut files: Vec<File>) -> Vec<File> {
    files.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    files
}

// TODO: colors should be customizable
pub fn print_files(files: Vec<File>) {
    for file in files {
        if file.is_folder {
            println!("{}", file.file_name.color("blue"))
        } else {
            println!("{}", file.file_name.color("white"))
        }
    }
}

// eye candy
pub fn get_files<T>(files: Rc<Vec<File>>, cond: T) -> Vec<File> 
where T: Fn(&&File) -> bool {
    files.iter().filter(cond).cloned().collect()
}

