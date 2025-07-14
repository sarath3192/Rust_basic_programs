use std::thread;
use std::thread::sleep;
use std::time;
fn test1(){
    for i in 1..5{
    println!("number:{}", i);
    }
}
fn main(){
    let s = String::from("I am string from main");
    let a = 10;
    
    // simple thread created with clouser
    thread::spawn(||{
        println!("Hello from thread");
    }).join().unwrap();

    // thread created with simple test1 function
    thread::spawn(test1).join().unwrap();

    // thread created with moving ownership of string from main to thread
    thread::spawn(move||{
        println!("move helps to take ownership from main: {}", &s);
    }).join().unwrap();

   // thread created with moving ownership but we use in main as it implements copy trait.
    thread::spawn(move ||{
        println!("{}", &a);
    }).join().unwrap();

    // Here we are passing a function pointer as argument to create a thread
    // thread::spawn(test1).join().unwrap();
    // println!("{}", s);
    let mut handles = vec![];
    for i in (1..5).rev(){
        println!("i: {}",i);
        let th = thread::spawn(move||{
            println!("From thread: {}", i);
            thread::sleep(time::Duration::from_secs(i));
            println!("thread {} closed", i);
        });
        handles.push(th);
    }
    
    for i in handles{
        i.join().unwrap();
    } 
    println!("All threads close success full");
    println!("{}", a);
}