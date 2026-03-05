fn total(matrix:&mut Vec<Vec<i32>>)->&Vec<Vec<i32>>{
    for inner_vec in matrix.iter_mut(){
        inner_vec.push(inner_vec.iter().sum());
    }
    let mut v: Vec<i32> = Vec::new();
    for col in 0..matrix[0].len()-1{
        let mut sum = 0;
        for row in 0..matrix.len(){
            sum = sum + matrix[row][col];
        }
        v.push(sum);
    }
    matrix.push(v);
    matrix
}
     
// Write a function, `reverse_in_place` that reverses an array/vector in-place
// Write a function, `reverse_str` that returns a reversed copy of a string
fn main() {
    let mut matrix:Vec<Vec<i32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![7, 8, 9, 10, 11],
    ];

    println!("before {:?}", matrix);
    println!("before {:?}", total(&mut matrix));
}
         

