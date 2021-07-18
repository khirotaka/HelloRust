use std::thread;
use std::time::Duration;


fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    // println!("press enter key");
    // std::io::stdin().read(&mut [0]);
    println!("please wait.");
    handle.join().expect("cannot join thread");
    println!("program end");

    let handle1 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #2 count {}.", i);
            thread::sleep(Duration::from_millis(2000));
        }
    });
    let handle3 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #3 count {}.", i);
            thread::sleep(Duration::from_millis(1500));
        }
    });

    println!("please wait.");
    handle1.join().expect("cannot join thread");
    handle2.join().expect("cannot join thread");
    handle3.join().expect("cannot join thread");
    println!("program end.");
}
