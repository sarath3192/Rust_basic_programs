use core::time;
use std::sync::Mutex;
use std::thread;
// use std::time;
use std::sync::Arc;
fn main(){
    let a = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5{
        // println!("data: {}", *data.lock().unwrap());
        let handle = thread::spawn(move||{
                let data = Arc::clone(&a);
                let mut b = data.lock().unwrap();
                println!("before: {}",b);
                *b +=1;
                println!("after: {}", b);        
        });
        thread::sleep(time::Duration::from_secs(1));
        handles.push(handle);
    }

    for hand in handles{
        hand.join().unwrap();
    }
}