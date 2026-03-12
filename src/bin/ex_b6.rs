use anyhow::{Context, Result, bail, anyhow};
use std::{fs, io, path::Path};

mod services {
    use regex::Regex;

    pub fn grep_content(content: &str, pattern: Regex) -> anyhow::Result<()> {
        for line in content.lines(){         
            match pattern.is_match(&line) {
                true  => println!("matched: {}", line),
                false => println!("no match: {}", line),
            }
         
        }

        Ok(())    
    }
}

fn main() -> anyhow::Result<()>{
    let path =  std::env::args().nth(1).context("specify path")?;
    let mut entries = fs::read_dir(&path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    println!("{:?}",entries);

    Ok(())
}
         
