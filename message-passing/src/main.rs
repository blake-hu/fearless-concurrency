use std::{sync::mpsc, thread};

fn main() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let my_sum = vec_sum(&v);
    assert_eq!(v.iter().sum::<i32>(), my_sum);
    println!("vec_sum test passed!")
}

fn vec_sum(v: &Vec<i32>) -> i32 {
    // multi-producer single-consumer channel
    // crucially, this is THREAD-SAFE
    let (sender, receiver) = mpsc::channel::<i32>();

    let chunks: Vec<&[i32]> = v.chunks(2).collect();

    // a thread::scope is basically a fork/join block.
    // all threads spawned within the scope are automatically joined by
    // the end of the scope.
    thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(|| {
                // fork
                println!("spawning thread!");
                let mut total = 0;
                for num in chunk.iter() {
                    total += num;
                }
                sender.send(total).expect("Should not fail to send total");
            });
        }
    }); // join all threads

    drop(sender); // disconnect sender so that we can drain receiver

    // get final total
    let mut total = 0;
    loop {
        match receiver.try_recv() {
            Ok(value) => total += value,
            Err(mpsc::TryRecvError::Empty) => {} // keep waiting
            Err(mpsc::TryRecvError::Disconnected) => break,
        }
    }
    total
}
