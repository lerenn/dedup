extern crate blake2;

use self::blake2::{Blake2b, Digest};
use std::{fs, io};

pub struct File {
    path: String,
    size: u64,
    hash: Option<&[u8]>
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

    pub fn hash(&mut self) -> &[u8] {
        match self.hash {
            None => {
                let hash = self.compute_hash().unwrap()
                self.hash = Some(hash);
                self.hash()
            },
            Some(hash) => hash,
        }
    }

    pub fn compute_hash(&self) -> std::io::Result<(&[u8])> {
        let mut file = fs::File::open(&self.path)?;
        let mut hasher = Blake2b::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = hasher.result().as_slice();
        Ok(hash)
    }
}
