extern crate blake2;

use self::blake2::{Blake2b, Digest};
use std::{fs, io};

pub struct File {
    path: String,
    size: u64,
    hash: Option<[u8; 64]>
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

    pub fn hash(&mut self) -> [u8; 64] {
        match self.hash {
            None => {
                // Calculate hash 
                self.hash = Some(self.compute_hash().unwrap());

                // Return result
                self.hash()
            },
            Some(hash) => hash,
        }
    }

    fn compute_hash(&self) -> std::io::Result<([u8; 64])> {
        let mut file = fs::File::open(&self.path)?;
        let mut hasher = Blake2b::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = hasher.result();

        // Return hash as array
        let mut out_hash: [u8; 64] = [0; 64];
        out_hash[..64].clone_from_slice(hash.as_slice());
        Ok(out_hash)
    }

    pub fn compare_hash(&mut self, file: &mut File) -> bool {
        // Get hashes
        let self_hash = self.hash();
        let file_hash = file.hash();

        for i in 0..64 {
            if self_hash[i] != file_hash[i] {
                return false;
            }
        }

        true
    }
}
