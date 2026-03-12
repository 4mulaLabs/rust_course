// Program 2: Takes 2 int args (run with: cargo run --bin ex2 -- 10 20)
fn ex2(arg1: u32, op: &str, arg2: u32) -> u32 {
    match op {
        "+" => arg1 + arg2,
        "-" => arg1 - arg2,
        "*" => arg1 * arg2,
        "/" => arg1 / arg2,
        _ => panic!("Invalid operator"),
    }
}


fn main() {
    let arg1 = std::env::args().nth(1).unwrap();
    let arg1_number = arg1.parse::<u32>().unwrap();
    let op = std::env::args().nth(2).unwrap();
    let arg2 = std::env::args().nth(3).unwrap();
    let arg2_number = arg2.parse::<u32>().unwrap();
    println!("{} {} {} = {}", arg1, op, arg2, ex2(arg1_number, &op, arg2_number));
}
