mod parameters;

use parameters::Parameters;

fn main() {
    let mut params = Parameters::new();
    params.parse();
    println!("Hello, world! {} {}",
        params.untouched_directory(),
        params.pruned_directory());
}
