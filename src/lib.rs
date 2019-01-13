mod directory;
mod file;

use directory::Directory;

pub struct Dedup {
    untouched_directory: Directory,
    pruned_directory: Directory,
}

impl Dedup {
    pub fn new(untouched_directory: &str, pruned_directory: &str) -> Dedup {
        Dedup {
            untouched_directory: Directory::new(untouched_directory),
            pruned_directory: Directory::new(pruned_directory),
        }
    }

    pub fn clean(&mut self) {
        self.pruned_directory.remove_duplicated_files(&mut self.untouched_directory);
        self.pruned_directory.remove_empty_directories();
    }
}
