use anyhow::{Context, Result, bail, anyhow};

mod services {
    
    pub fn reverse_file_content(content:&str)->anyhow::Result<String>{
        
        Ok(content.lines().rev().collect::<Vec<&str>>().join("\n"))
    }
}

fn main() -> anyhow::Result<()>{
   let filename =  std::env::args().nth(1).context("specify filename")?;

   println!("filename is - {}",filename);
   
   let content = std::fs::read_to_string(&filename)?;
   
   let reversed: String = services::reverse_file_content(&content)?;

   println!("{}",reversed);
   
   Ok(())
}
         
