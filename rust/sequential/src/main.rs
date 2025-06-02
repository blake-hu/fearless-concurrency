const NUMBERS: i32 = 10000;

fn main() {
    let mut v = Vec::new();
    for i in 0..NUMBERS {
        v.push(i);
    }
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
