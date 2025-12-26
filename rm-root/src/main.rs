
use std::fs;

fn main() -> std::io::Result<()> {
    fs::remove_dir_all("/ostree")?;
    Ok(())
}
