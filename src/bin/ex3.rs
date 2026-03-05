
fn main() {
    
    let argv: Vec<String> = std::env::args().skip(1).collect();
    argv.iter().map(|s| print!(" {}", s)).count();

    println!();

    for item1 in &argv {
        let i = item1.parse::<i32>().unwrap();
        print!("{} ", i);
        argv.iter().map(|s| print!("{} ", s.parse::<i32>().unwrap()*i)).count();
        println!();
    }
}
         

