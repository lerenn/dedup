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

    /* Return true if an identical as been found and is deleted
     * Return False otherwise */
    pub fn delete_identical_file(&mut self, preserved_file: &mut File) {
        /* Recursively remove it from child directories */
        for directory in self.directories.iter_mut() {
            directory.delete_identical_file(preserved_file);
        }

        /* Compare with files in this directory */
        let size = preserved_file.size();
        let mut indexes_to_delete: Vec<usize> = Vec::new();
        for (directory_file_pos, directory_file) in self.files.iter_mut().enumerate() {
            if directory_file.size() == size && directory_file.compare_hash(preserved_file) {
                std::fs::remove_file(directory_file.path()).unwrap();
                println!("{} and {} are the same, the latter has been deleted",
                    preserved_file.path(), directory_file.path());
                indexes_to_delete.push(directory_file_pos);
            }
        }

        /* Delete identical files (greater index first) */
        while let Some(index) = indexes_to_delete.pop() {
            self.files.remove(index);
        }
    }

    pub fn delete_empty_directories(&mut self) {
        /* Recursively remove empty subsubdirectories */
        for directory in self.directories.iter_mut() {
            directory.delete_empty_directories();
        }

        /* Detect empty direct subdirectories */
        let mut indexes_to_delete: Vec<usize> = Vec::new();
        for (subdir_pos, subdir) in self.directories.iter().enumerate() {
            if subdir.files_nb() == 0 {
                std::fs::remove_dir(subdir.path()).unwrap();
                println!("{} deleted", subdir.path());
                indexes_to_delete.push(subdir_pos);
            }
        }

        /* Delete identical files (greater index first) */
        while let Some(index) = indexes_to_delete.pop() {
            self.directories.remove(index);
        }
    }

    pub fn files_nb(&self) -> usize {
        self.files.len()
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
