use anyhow::{Context, Result, bail, anyhow};
use regex::Regex;

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
    let filename =  std::env::args().nth(1).context("specify filename")?;
    let pattern =  std::env::args().nth(2).context("specify pattern")?;
    
    let pattern: Regex = Regex::new(&pattern).unwrap();
   
   let content = std::fs::read_to_string(&filename)?;
   
   services::grep_content(&content,pattern)?;

   Ok(())
}
         
