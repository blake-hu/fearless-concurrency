use std::thread;

const NUMBERS: i32 = 10000;
const CHUNK_SIZE: usize = 100;

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
    // Arc is an Atomically Reference-Counting pointer
    let mut total = 0;
    let chunks: Vec<&[i32]> = v.chunks(CHUNK_SIZE).collect();

    // a thread::scope is basically a fork/join block.
    // all threads spawned within the scope are automatically joined by
    // the end of the scope.
    thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(|| {
                // fork
                for num in chunk.iter() {
                    total += num;
                }
            });
        }
    }); // join all threads

    // get final total
    total
}
