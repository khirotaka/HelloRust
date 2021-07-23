use std::thread;
use std::time::Duration;
use futures::executor::ThreadPool;
use std::io::Read;


fn main() {
    let pool = ThreadPool::new().unwrap();
    let task = async {
        let mut id = 0;
        for j in 1..6 {
            // let id = j * 10;
            id += 10;
            // async ブロックの外側にる変数を使う場合は
            // move キーワードをつけてあげる必要がある。
            // これで変数を内部で利用できるようになる。
            pool.spawn_ok(async move {
                for i in 0..10 {
                    println!("thread #{} count {}.", id, i);
                    id += 1;
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            thread::sleep(Duration::from_millis(500));
        }
    };

    println!("program start.");
    futures::executor::block_on(task);
    println!("press any key.");
    std::io::stdin().read(&mut [0]);
    println!("program end");
}
