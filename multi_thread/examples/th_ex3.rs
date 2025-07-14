use std::{sync::Arc, thread};

fn main(){
    let a = String::from("hello Workd");
    let k = Arc::new(a);
    let mut handles = vec![];
    for _ in 0..5{
        let x = Arc::clone(&k);
        println!("clone_count: {}", Arc::strong_count(&k));
        let handle = thread::spawn(move||{
            println!("{}", x);
        });
        handles.push(handle);
    }
    for i in handles{
        i.join().unwrap();
    }
}