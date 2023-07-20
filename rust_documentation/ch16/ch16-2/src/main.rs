use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = "hi".to_string();
        println!("Send: {}", val);
        tx.send(val).unwrap(); 
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
}
