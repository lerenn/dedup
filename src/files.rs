use std::path::Path;

struct File<'a> {
    relative_path: String,
    size: u64,
    hash: Option<u64>,
    parent: &'a Directory<'a>,
}

impl<'a> File<'a> {
    pub fn new(path: &str, parent: &'a Directory<'a>) -> File<'a> {
        File {
            relative_path: String::from(path),
            size: 0,
            hash: None,
            parent,
        }
    }
}

pub struct Directory<'a> {
    relative_path: String,
    children: Vec<File<'a>>,
    parent: Option<&'a Directory<'a>>,
}

impl<'a> Directory<'a> {
    pub fn new(path: String) -> Directory<'a> {
        /* Set directory */
        let mut dir = Directory {
            relative_path: path.clone(),
            children: Vec::new(),
            parent: None,
        };

        /* Get children */
        for entry in Path::new(&path).read_dir().unwrap() {
            let child_path = entry.unwrap().path();
            let child = Path::new(&child_path);
            if child.is_file() == true {
                let child_path_str = child_path.to_str().unwrap();
                // dir.children.push(File::new(child_path_str, &dir));
                println!("{}", child_path_str);    
            }
        }

        /* Return value */
        dir
    }
}
