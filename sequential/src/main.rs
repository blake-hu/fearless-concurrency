fn main() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let my_sum = vec_sum(&v);
    assert_eq!(v.iter().sum::<i32>(), my_sum);
    println!("vec_sum test passed!")
}

fn vec_sum(v: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in v.iter() {
        total = total + num;
    }
    total
}
