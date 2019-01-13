pub struct File {
    path: String,
    size: u64,
    hash: Option<u64>
}

impl File {
    pub fn new(path: &str) -> File {
        File {
            path: String::from(path),
            size: 0,
            hash: None,
        }
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
