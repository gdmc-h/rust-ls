use std::{fs::DirEntry, io::Error};

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Default)]
pub struct File {
    pub file_name: String,
    pub is_folder: bool,
    pub permissions: u32,
    pub size: u32
}

impl File {
    fn get_permissions() {
        todo!()
    }
    
    fn get_size() {
        todo!()
    }

    pub fn fetch_file(dir: Result<DirEntry, Error>) -> Self {
        let unwrap_dir: DirEntry = match dir {
            Ok(ok) => ok,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let file_name = String::from(
            match unwrap_dir.file_name().to_str() {
                Some(name) => name,
                None => panic!("Panic: this was not supposed to happen")
            }
        );

        let is_folder = match unwrap_dir.file_type() {
            Ok(file) => file.is_dir(),
            Err(e) => panic!("{}", e)
        };

        Self {
            file_name,
            is_folder,
            permissions: 0,// read file permission
            size: 0 // read file size
        }
    }
}
