// Write a function, `reverse` that reverses a sequance of numbers (array or vector) by returning a reversed copy of it
use std::ptr;

fn reverse(arr: &[i32]) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for i in (0..arr.len()).rev(){
        vec.push(arr[i]);
    }
    vec
}

fn reverse_in_place(arr: &mut[i32]) -> &[i32] {
    let mut i = 0;
    let mut j = arr.len();
    j -= 1;
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
    arr
}


fn reverse_str(s: &String) -> String {
    let reversed: String = s.chars().rev().collect();
    reversed
}
     
// Write a function, `reverse_in_place` that reverses an array/vector in-place
// Write a function, `reverse_str` that returns a reversed copy of a string
fn main() {
    let mut arr: [i32;5] = [1,2,3,4,5];
    let before_ptr = arr.as_ptr();
    let arr_1 = reverse(&arr);
    println!("reverse >> {:?}",arr_1);

    let arr_2 = reverse_in_place(&mut arr);
    println!("reverse_in_place >> {:?}",arr_2);
    println!("same object a/b: {}", ptr::eq(before_ptr, arr_2.as_ptr())); // should be true

    let s = String::from("Hello");
    println!("reverse_str >> {:?}",reverse_str(&s));
}
         

