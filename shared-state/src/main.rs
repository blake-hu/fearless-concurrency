use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let my_sum = vec_sum(&v);
    assert_eq!(v.iter().sum::<i32>(), my_sum);
    println!("vec_sum test passed!")
}

fn vec_sum(v: &Vec<i32>) -> i32 {
    // Arc is an Atomically Reference-Counting pointer
    let total_arc = Arc::new(RwLock::new(0));
    let chunks: Vec<&[i32]> = v.chunks(2).collect();

    // a thread::scope is basically a fork/join block.
    // all threads spawned within the scope are automatically joined by
    // the end of the scope.
    thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(|| {
                // fork
                println!("spawning thread!");
                for num in chunk.iter() {
                    let mut total = total_arc
                        .write()
                        .expect("Lock should not be poisoned or double locked");
                    *total += num;
                }
            });
        }
    }); // join all threads

    // get final total
    *total_arc
        .read()
        .expect("Lock should not be poisoned or double-locked")
}
