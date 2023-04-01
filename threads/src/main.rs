use std::{
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

fn main() {
    println!("This will be printed");
    println!("this will also be printed");
    println!("the concurrency will start after this");

    // blocking the execution until the thread is complete

    let t: JoinHandle<()> = thread::spawn(|| {
        println!("blocking the execution until this thread completes");
        println!("blocking the excution until this thread completes 2");

        println!("blocking the excution until this thread completes 3");
        println!("blocking the excution until this thread completes 4");
        println!("blocking the excution until this thread completes 5");
        println!("blocking the excution until this thread completes 6");
        println!("blocking the excution until this thread completes 7");
        println!("blocking the excution until this thread completes 8");
        println!("blocking the excution until this thread completes 9");
    });
    t.join();

    sleep(Duration::from_secs(10));
    thread::spawn(|| {
        println!("Hello 1 from the thread");
        println!("Hello 8 from the thread");
        println!("Hello 7 from the thread");
        println!("Hello 6 from the thread");
        println!("Hello 5 from the thread");
        println!("Hello 4 from the thread");
        println!("Hello 3 from the thread");
        println!("Hello 2 from the thread");
        println!("Hello 1 from the thread");
    });

    println!("hello 1 from the main");
    println!("Hello 2 from the main");

    sleep(Duration::from_secs(10));

    // ownership in thread
    //

    let mut thread_vec: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        thread_vec.push(thread::spawn(move || {
            println!("thread number {}", i);
        }))
    }
}
