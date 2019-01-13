extern crate dedup;
extern crate argparse;

use argparse::{ArgumentParser, Store};

pub struct Parameters {
    untouched_directory: String,
    pruned_directory: String,
}

impl Parameters {
    pub fn new() -> Parameters {
        Parameters {
            untouched_directory: String::from(""),
            pruned_directory: String::from(""),
        }
    }

    pub fn parse(&mut self) {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut self.untouched_directory)
            .add_argument(&"untouched_directory", Store,
            "Untouched directory");
        ap.refer(&mut self.pruned_directory)
            .add_argument(&"pruned_directory", Store,
            "Directory to prune");
        ap.parse_args_or_exit();
        ap.set_description("This utilitary will find redundant files between two directories (or not).");
    }

    pub fn untouched_directory(&self) -> &String {
        &self.untouched_directory
    }

    pub fn pruned_directory(&self) -> &String {
        &self.pruned_directory
    }
}

fn main() {
    /* Get parameters */
    let mut params = Parameters::new();
    params.parse();
    println!("Dedup will compare {} and {}, the latter will be pruned.",
        params.untouched_directory(),
        params.pruned_directory());

    /* Directories */
    let _dd = dedup::new(params.untouched_directory(), params.pruned_directory());
}
