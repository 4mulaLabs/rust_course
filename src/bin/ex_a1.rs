fn ex1(arr: &[i32]) -> i32 {
    for i in 0..arr.len() {
        let mut sum_left = 0;
        let mut sum_right = 0;
        for j in 0..i {
            sum_left += arr[j];
        }
        for j in i + 1..arr.len() {
            sum_right += arr[j];
        }
        if sum_left == sum_right {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("Pivot index: {}", ex1(&[1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]));
}
