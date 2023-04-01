#[tokio::main]
async fn main() {
    let mut handles = vec![];

    println!("this code is not the part of the async block");

    let s1 = String::from("Huge computation function");
    let s2 = String::from("Simpler computation function");

    let aw1 = tokio::spawn(async move {
        huge_computation(s1).await;
    });

    handles.push(aw1);

    let aw2 = tokio::spawn(async move {
        simpler_computation(s2).await;
    });

    handles.push(aw2);

    for handle in handles {
        handle.await.unwrap();
    }

    // selecting a function that returns first

    let aw1 = tokio::spawn(async move {
        function_1().await;
    });

    let aw2 = tokio::spawn(async move {
        function2().await;
    });

    tokio::select! {
        _=aw1=>println!("Function1 is selected"),
        _=aw2=> println!("Function 2 is selected"),
    };

    // this will ensure that both the funtions are executed

    tokio::join! {
        function_1(),
        function2(),
    };

    println!("all tasks are now completed");
}

async fn function_1() {
    for _ in 1..100_000_000 {}
}

async fn function2() {}

async fn huge_computation(s: String) {
    println!("{:?} has started", s);

    for _ in 0..100_000_000 {}
    println!("{:?} is now completed", s);
}

async fn simpler_computation(s: String) {
    println!("{:?} has started", s);
    println!("{:?} has ended", s);
}
