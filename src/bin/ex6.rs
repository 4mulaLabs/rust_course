


fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut count:u32 = 0 ;
    let mut last:i32 = -1;
    for item in &argv{
        let current:i32 =  item.parse().unwrap();
        dbg!(current);
        if last == -1{
            last = current;
            count = 1;
        }else if  last == current{
            count += 1;
        }else{
            print!("{} {} ",count,last);
            last = current;
            count = 1;
        }
    }
    println!("{} {}",count,last);
}
         
