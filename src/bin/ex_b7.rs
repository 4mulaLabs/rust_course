use anyhow::{Context, Result, bail, anyhow};
use std::path::Path;

mod services {
    use std::fs::{self, DirEntry};
    use std::path::Path;

    pub fn print_entry(entry: &DirEntry) {
        println!("{} {}", entry.path().file_name().unwrap().to_string_lossy(), entry.path().display());
    }
    pub fn visit_dirs(dir: &Path, filename:&str,cb: &dyn Fn(&DirEntry)) -> anyhow::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, filename,cb)?;
                } else if filename == entry.path().file_name().unwrap(){
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()>{
    let path =  std::env::args().nth(1).context("specify path")?;
    let filename =  std::env::args().nth(2).context("specify filename")?;
    let dir = Path::new(&path);  // current directory

    services::visit_dirs(dir, &filename, &services::print_entry)?;

    Ok(())
}
         
