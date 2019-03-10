use file::File;

use std::path::Path;
use std::fs::metadata;

pub struct Directory {
    path: String,
    files: Vec<File>,
    directories: Vec<Directory>,
    dry_run: bool,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        // Set directory 
        let mut dir = Directory {
            path: String::from(path),
            files: Vec::new(),
            directories: Vec::new(),
            dry_run: false,
        };

        // Get children 
        for entry in Path::new(&path).read_dir().unwrap() {
            let child_path = entry.unwrap().path();
            let child_metadata = metadata(&child_path).unwrap();

            // Add this child 
            let child_path_str = child_path.to_str().unwrap();
            if child_metadata.is_file() == true { // As file 
                dir.files.push(File::new(child_path_str, child_metadata.len()));
            } else { // As directory 
                dir.directories.push(Directory::new(child_path_str));
            }
        }

        // Return value 
        dir
    }

    pub fn delete_identical_files_from(&mut self, compared_directory: &mut Directory) {
        // Recursively remove it to child directories 
        for directory in self.directories.iter_mut() {
            directory.delete_identical_files_from(compared_directory);
        }

        // Compare with files in this directory 
        for file in self.files.iter_mut() {
            compared_directory.delete_identical_file(file);
        }
    }

    fn remove_file(&mut self, index: usize) {
        // Get path
        let path = self.files[index].path().clone();
        println!("{} deleted", path);

        // Remove file from SysFS
        if self.dry_run == false {
            std::fs::remove_file(path).unwrap();
        }

        // Remove from internal FS
        self.files.remove(index);
    }

    // Will only works if the directory is empty !!
    fn remove_dir(&mut self, index: usize) {
        // Get path
        let path = self.directories[index].path().clone();
        println!("{} deleted", path);

        // Remove file from SysFS
        if self.dry_run == false {
            std::fs::remove_dir(path).unwrap();
        }

        // Remove from internal FS
        self.directories.remove(index);
    }

    // Return true if an identical as been found and is deleted
    // Return False otherwise
    pub fn delete_identical_file(&mut self, preserved_file: &mut File) {
        // Recursively remove it from child directories 
        for directory in self.directories.iter_mut() {
            directory.delete_identical_file(preserved_file);
        }

        // Compare with files in this directory 
        let size = preserved_file.size();
        let mut indexes_to_delete: Vec<usize> = Vec::new();
        for (directory_file_pos, directory_file) in self.files.iter_mut().enumerate() {
            if directory_file.size() == size && directory_file.compare_hash(preserved_file) {
                println!("{} == {}", preserved_file.path(), directory_file.path());
                indexes_to_delete.push(directory_file_pos);
            }
        }

        // Delete identical files (greater index first) 
        while let Some(index) = indexes_to_delete.pop() {
            self.remove_file(index);
        }
    }

    pub fn delete_empty_directories(&mut self) {
        // Recursively remove empty subsubdirectories 
        for directory in self.directories.iter_mut() {
            directory.delete_empty_directories();
        }

        // Detect empty direct subdirectories 
        let mut indexes_to_delete: Vec<usize> = Vec::new();
        for (subdir_pos, subdir) in self.directories.iter().enumerate() {
            if subdir.files_nb() == 0 {
                println!("{} is empty", subdir.path());
                indexes_to_delete.push(subdir_pos);
            }
        }

        // Delete identical files (greater index first) 
        while let Some(index) = indexes_to_delete.pop() {
            self.remove_dir(index);
        }
    }

    pub fn files_nb(&self) -> usize {
        self.files.len()
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn set_dry_run(&mut self, value: bool) {
        // Set dry run to subdirectories
        for dir in self.directories.iter_mut() {
            dir.set_dry_run(value);
        }

        // Set dry run to itself
        self.dry_run = value;
    }
}
