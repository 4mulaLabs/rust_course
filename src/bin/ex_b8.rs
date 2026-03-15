fn total(matrix: &mut Vec<Vec<Option<i32>>>) -> &Vec<Vec<Option<i32>>> {
    for inner_vec in matrix.iter_mut(){

        let mut result = Some(0);

        for item in inner_vec.iter(){
            match item {
                Some(val) => {result = Some(result.unwrap() + val)},
                None => {result = None;break;},  
            }
        }

        match result{
            Some(val) => {inner_vec.push(Some(val))},
            None => {inner_vec.push(None)},         
        }
    }
    let mut v: Vec<Option<i32>> = Vec::new();
    for col in 0..matrix[0].len()-1{
        let mut sum = Some(0);
        for row in 0..matrix.len(){
            match matrix[row][col] {
                    Some(val) => {sum = Some(sum.unwrap() + val)},
                    None => {sum = None;break;},  
            }
        }
        v.push(sum);
    }
    matrix.push(v);
    matrix
}
     
// Write a function, `reverse_in_place` that reverses an array/vector in-place
// Write a function, `reverse_str` that returns a reversed copy of a string
fn main() {
    let mut matrix: Vec<Vec<Option<i32>>> = vec![
        vec![Some(1), None, Some(3), Some(4), Some(5)],
        vec![Some(2), Some(3), Some(4), None, Some(6)],
        vec![Some(7), Some(8), Some(9), Some(10), Some(11)],
    ];

    println!("before:");
    for row in &matrix{
        println!("{:?}",row);
    }
    println!("after");
    for row in total(&mut matrix){
        println!("{:?}",row);
    }
}
         

