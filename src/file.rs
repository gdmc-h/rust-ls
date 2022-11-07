use colored::*;

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone)]
pub struct File {
    pub file_name: String,
    pub is_folder: bool,
    pub permissions: u32,
    pub size: u32
}

#[derive(Default)]
pub struct Path {
    pub folders: Vec<File>,
    pub files: Vec<File>
}


impl Path {
    fn set_files<T, A>(&mut self, files: &[File], filter_condition_folder: T, filter_condition_file: A)
    where T: Fn(&&File) -> bool,
          A: Fn(&&File) -> bool { // there's probably a better solution to this C-c/C-v
        self.files = files.iter().filter(filter_condition_file).cloned().collect();
        self.folders = files.iter().filter(filter_condition_folder).cloned().collect();
    }

    fn sort_files(&mut self) {
        self.files.sort_by(|a, b| a.file_name.cmp(&b.file_name));
        self.folders.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    }

    fn print_files(&self) {
        let print_it = |file: &File| {
            if file.is_folder {
                println!("{}", file.file_name.color("blue"))
            } else {
                println!("{}", file.file_name.color("white"))
            }
        };

        self.folders.iter().for_each(print_it);
        self.files.iter().for_each(print_it);
    }

    pub fn show_path(&mut self, files: &[File]) {
        self.set_files(files, |p| p.is_folder, |a| !a.is_folder);
        self.sort_files();
        self.print_files();
    }
}



