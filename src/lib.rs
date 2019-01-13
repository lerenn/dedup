mod directory;
mod file;

use directory::Directory;

pub struct Dedup {
    untouched_directory: Directory,
    pruned_directory: Directory,
}

pub fn new(untouched_directory: &str, pruned_directory: &str) -> Dedup {
    Dedup {
        untouched_directory: directory::new(untouched_directory),
        pruned_directory: directory::new(pruned_directory),
    }
}
