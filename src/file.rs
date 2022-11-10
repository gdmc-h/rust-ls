use std::{fs::DirEntry, io::Error, os::unix::prelude::PermissionsExt};

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Default)]
pub struct File {
    pub file_name: String,
    pub is_folder: bool,
    pub permissions: String,
    pub size: String
}

impl File {
    fn get_size(size: u64) -> String {
        let f64_size = f64::from(size as u32);

        (f64_size / 10000.0).to_string()
    }

    fn get_permissions(mode: u32) -> String {
        let full_metadata_mode = format!("{:o}", mode);
        let chars_to_show = full_metadata_mode.len()-3;

        full_metadata_mode[chars_to_show..].to_string()
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

        let file_metadata = match unwrap_dir.metadata() {
            Ok(metadata) => metadata,
            Err(e) => panic!("{}", e)
        };

        let permissions = File::get_permissions(file_metadata.permissions().mode());
        let size = File::get_size(file_metadata.len());

        Self {
            file_name,
            is_folder,
            permissions,
            size 
        }
    }
}
