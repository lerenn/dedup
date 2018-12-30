use std::fs::{File, remove_dir_all, create_dir};
use std::io::ErrorKind::NotFound;
use std::io::{Write};

fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn setup() -> std::io::Result<()> {
    clean("tests/data");
    create_dir("tests/data")?;

    /* Write directory A */
    create_dir("tests/data/A")?;
    write_file("tests/data/A/A.log", "AAAA")?;
    write_file("tests/data/A/B.log", "BBBB")?;

    /* Write directory B */
    create_dir("tests/data/B")?;
    write_file("tests/data/B/A.log", "AAAA")?;
    write_file("tests/data/B/C.log", "CCCC")?;
    Ok(())
}

fn clean(path: &str) {
    /* Remove data test */
    match remove_dir_all(path) {
        Ok(_) => { /* Nothing to do */ },
        Err(err) => {
            match err.kind() {
                NotFound => { /* Nothing to do */ },
                err => {
                    panic!("There has been an error when deleting directory {}: {:?}", path, err);
                }
            }
        }
    };
}
