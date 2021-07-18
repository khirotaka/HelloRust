use std::thread;
use std::time::Duration;


async fn foo(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

async fn sub() {
    foo(10).await;
    foo(20).await;
    foo(30).await;
}

/*
fn main() {
    println!("program start");
    futures::executor::block_on(sub());
    println!("program end");
}
*/

/*
# tokio を使う例 1
#[tokio::main]
async fn main() {
    println!("program start");
    foo(10).await;
    foo(20).await;
    foo(30).await;
    println!("program end");
}
*/

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    println!("program start");
    rt.block_on(async {
        foo(10).await;
        foo(20).await;
        foo(30).await;
    });
    println!("program end");
}