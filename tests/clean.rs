extern crate dedup;

use dedup::Dedup;

mod common;

#[test]
fn normal_clean() {
    /* Setup environment */
    common::setup().unwrap();

    /* Create filer */
    let mut filer = Dedup::new("tests/data/A", "tests/data/B");

    /* Remove conflicting files are present */
    filer.clean();
}
