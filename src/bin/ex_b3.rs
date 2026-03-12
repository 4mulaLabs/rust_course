use anyhow::{Context, Result, bail, anyhow};

mod services {
    
    pub fn filter_content(content:&str,col:&Vec<u32>)->anyhow::Result<String>{
        let mut result: Vec<String> = Vec::new();
        for line in content.lines(){         
            let  words: Vec<&str> = line.split_whitespace().collect();
            let mut filter_words:  Vec<String> = Vec::new();
            let mut pos = 1;
            for word in words{
                if col.contains(&pos) {
                    filter_words.push(word.into());
                }
                pos += 1;
            }
            result.push(filter_words.join(" "));
        }

        Ok(result.join("\n"))
    }
}

fn main() -> anyhow::Result<()>{
    let mut args = std::env::args();
    
    args.next(); // skip the program name (args[0])
    
    let filename = args.next().context("specify filename")?;
    
    let other_args: Vec<String> = args.collect(); // all remaining args
    
    let mut numbers: Vec<u32> = Vec::new();
    for arg in other_args {
        let n: u32 = arg.parse().context("arguments must be positive integers")?;
        numbers.push(n);
    }
   
   let content = std::fs::read_to_string(&filename)?;
   
   let filter_content: String = services::filter_content(&content,&numbers)?;

   println!("{}",filter_content);

   Ok(())
}
         
