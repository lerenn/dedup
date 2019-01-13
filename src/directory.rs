use file::File;

use std::path::Path;

pub struct Directory {
    path: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        /* Set directory */
        let mut dir = Directory {
            path: String::from(path),
            files: Vec::new(),
            directories: Vec::new(),
        };

        /* Get children */
        for entry in Path::new(&path).read_dir().unwrap() {
            let child_path = entry.unwrap().path();
            let child = Path::new(&child_path);

            /* Add this child */
            let child_path_str = child_path.to_str().unwrap();
            if child.is_file() == true { /* As file */
                dir.files.push(File::new(child_path_str));
            } else { /* As directory */
                dir.directories.push(Directory::new(child_path_str));
            }
        }

        /* Return value */
        dir
    }

    pub fn list_files(&self) {
        /* See for directories */
        for directory in self.directories.iter() {
            directory.list_files();
        }

        /* See for files */
        for file in self.files.iter() {
            println!("{}", file.path());
        }
    }

    pub fn remove_duplicated_files(&mut self, compared_directory: &mut Directory) {
        self.list_files(); /* TO REMOVE */
        /* TODO */
    }

    pub fn remove_empty_directories(&mut self) {
        /* TODO */
    }
}
