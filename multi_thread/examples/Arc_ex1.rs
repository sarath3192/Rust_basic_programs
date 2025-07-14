use std::sync::{Arc, Mutex};
fn main(){
    let a = Arc::new(Mutex::new(0)); // 1st owner
    // println!("Reference count: {}", Arc::strong_count(&a));
    println!("{}", *a.lock().unwrap());
    let b = Arc::clone(&a);
    *b.lock().unwrap() +=2;
    *b.lock().unwrap() *=3;
    println!("{}",*b.lock().unwrap());

    // let b = Arc::clone(&a); // second owner
    // println!("{:?}", b);
    // let c = Arc::clone(&a); // third owner
    // let d = Arc::clone(&a); // forth owner
    // println!("Refence count: {}", Arc::strong_count(&a)); 
}