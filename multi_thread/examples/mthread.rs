use std::thread;
use std::time::Duration;
use std::thread::sleep;

fn func_th()->(){
    println!("From: func th")
}

fn test1()-> (){
    println!("Hello world")
}
fn main(){
    
    let a = thread::spawn(test1);
        // println!("timer started");
        // sleep(Duration::from_secs(10));
        // println!("timer ended");
    
    a.join().unwrap(); // This statement make the program to wait untill the thread is finished.
                           // To make sure that main is not finished before thread.
    println!("Thread is not finished");
    // a.join().unwrap();
    println!("Main is going to end now")
}