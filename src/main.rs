mod parameters;
mod files;

use parameters::Parameters;
use files::Directory;

fn main() {
    /* Get parameters */
    let mut params = Parameters::new();
    params.parse();
    println!("Dedup will compare {} and {}, the latter will be pruned.",
        params.untouched_directory(),
        params.pruned_directory());

    /* Directories */
    let _untouched_filer = Directory::new(params.untouched_directory().to_string(), None);
}
