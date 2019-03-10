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
        self.untouched_directory.delete_identical_files_from(&mut self.pruned_directory);
        self.pruned_directory.delete_empty_directories();
    }

    pub fn set_dry_run(&mut self, value: bool) {
        self.untouched_directory.set_dry_run(value);
        self.pruned_directory.set_dry_run(value);
    }
}
