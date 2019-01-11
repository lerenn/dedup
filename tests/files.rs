extern crate dedup;

mod common;

#[test]
fn list_files() {
    /* Setup environment */
    common::setup().unwrap();

    /* Create filer */
    let _filer = dedup::Directory::new("tests/data");
}
