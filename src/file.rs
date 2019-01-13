pub struct File {
    relative_path: String,
    size: u64,
    hash: Option<u64>
}

pub fn new(path: &str) -> File {
    File {
        relative_path: String::from(path),
        size: 0,
        hash: None,
    }
}
