use directory::Directory;

pub struct File {
    path: String,
    size: u64,
    hash: Option<u64>
}

impl File {
    pub fn new(path: &str, size: u64) -> File {
        File {
            path: String::from(path),
            size,
            hash: None,
        }
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn hash(&mut self) -> u64 {
        match self.hash {
            None => {
                self.hash = Some(self.compute_hash());
                self.hash()
            },
            Some(hash) => hash,
        }
    }

    pub fn compute_hash(&self) -> u64 {
        0 /* TODO */
    }
}
