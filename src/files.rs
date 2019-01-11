use std::path::Path;

struct File {
    relative_path: String,
    size: u64,
    hash: Option<u64>
}

impl File {
    pub fn new(path: &str) -> File {
        File {
            relative_path: String::from(path),
            size: 0,
            hash: None,
        }
    }
}

pub struct Directory {
    relative_path: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
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
                dir.files.push(File::new(child_path_str));
            } else { /* As directory */
                dir.directories.push(Directory::new(child_path_str));
            }
        }

        /* Return value */
        dir
    }
}
