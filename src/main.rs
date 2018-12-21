extern crate argparse;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut untouched_directory = String::new();
    let mut pruned_directory = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut untouched_directory)
            .add_argument(&"untouched_directory", Store,
            "Untouched directory");
        ap.refer(&mut pruned_directory)
            .add_argument(&"pruned_directory", Store,
            "Directory to prune");
        ap.parse_args_or_exit();
        ap.set_description("This utilitary will find redundant files between two directories (or not).");
    }

    println!("Hello, world! {} {}", untouched_directory, pruned_directory);
}
