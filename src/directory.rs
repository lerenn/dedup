use file::File;

use std::path::Path;
use std::fs::metadata;

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
            let child_metadata = metadata(&child_path).unwrap();

            /* Add this child */
            let child_path_str = child_path.to_str().unwrap();
            if child_metadata.is_file() == true { /* As file */
                dir.files.push(File::new(child_path_str, child_metadata.len()));
            } else { /* As directory */
                dir.directories.push(Directory::new(child_path_str));
            }
        }

        /* Return value */
        dir
    }

    pub fn delete_identical_files_from(&mut self, compared_directory: &mut Directory) {
        /* Recursively remove it to child directories */
        for directory in self.directories.iter_mut() {
            directory.delete_identical_files_from(compared_directory);
        }

        /* Compare with files in this directory */
        for file in self.files.iter_mut() {
            compared_directory.delete_identical_file(file);
        }
    }

    pub fn delete_identical_file(&mut self, file: &mut File) {
        /* Recursively remove it from child directories */
        for directory in self.directories.iter_mut() {
            directory.delete_identical_file(file);
        }

        /* Compare with files in this directory */
        let size = file.size();
        for directory_file in self.files.iter_mut() {
            if directory_file.size() == size && directory_file.hash() == file.hash() {
                println!("{} and {} are the same", directory_file.path(), file.path());
            }
        }
    }

    pub fn delete_empty_directories(&mut self) {
        /* TODO */
    }
}
