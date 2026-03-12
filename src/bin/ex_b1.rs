use std::fs::read;

use anyhow::{Context, Result, bail, anyhow};

fn read_file_content(filename:&str)->Result<String>{

    Ok(String::new())
}
fn main() -> anyhow::Result<()>{
   let filename =  std::env::args().nth(1).context("specify filename")?;

   println!("filename is - {}",filename);
   
   let content = std::fs::read_to_string(&filename)?;
    
   println!("{}",content);

   Ok(())
}
         
