use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    // thread::spawn(task(5)); only closure
    thread::spawn(|| {
        for i in 1..200 {
            println!("thread");
        };
    });
    for i in 1..200 {
        println!("main");
    };


    let handler = thread::spawn(|| {
        for i in 1..200 {
            println!("thread");
        };
    });
    handler.join().unwrap();


    let v: Vec<i32> = vec![1, 2, 3];
    //error:cuz it know nothing about when v will dropped ouside.
    //thread::spawn(||{
    //    println!("{:?}",v);
    //});
    thread::spawn(move || {
        println!("{:?}", v);
    });
    // println!("{:?}",v);


    let (sender, receiver) = mpsc::channel();
    thread::spawn(move||{
        sender.send("123");
    });
    let msg=receiver.recv().unwrap();
    println!("{}",msg);



    let count=Arc::new(Mutex::new(0));

    for i in 1..10{
        let clone=Arc::clone(&count);
        thread::spawn(move ||{
            let mut m=clone.lock().unwrap();
            *m+=1;
        });
    }
    thread::sleep(Duration::new(2,1));
    println!("{:?}",count);
}

fn task(i: i32) {
    println!("{}", i);
}