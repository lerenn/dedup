use file;
use file::File;

use std::path::Path;

pub struct Directory {
    relative_path: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

pub fn new(path: &str) -> Directory {
    /* Set directory */
    let mut dir = Directory {
        relative_path: String::from(path),
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
            dir.files.push(file::new(child_path_str));
        } else { /* As directory */
            dir.directories.push(new(child_path_str));
        }
    }

    /* Return value */
    dir
}
