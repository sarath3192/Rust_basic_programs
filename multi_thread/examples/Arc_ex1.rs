use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Num{
    x: i32,
    y: i32,
}
fn main(){
    let a = Arc::new(Mutex::new(0)); // 1st owner
    println!("{}", *a.lock().unwrap());
    *a.lock().unwrap() +=1;
    println!("{}", *a.lock().unwrap());

    let b = Arc::clone(&a);
    *b.lock().unwrap() +=2;
    *b.lock().unwrap() *=3;
    println!("{}",*b.lock().unwrap());

    let c =  Arc::clone(&b);
    let d =  Arc::clone(&c);
    *d.lock().unwrap() = 10;
    println!("{}", *d.lock().unwrap());
    println!("{}", Arc::strong_count(&c));


    let e = Arc::clone(&a); // second owner
    println!("{:?}", b);
    let f = Arc::clone(&a); // third owner
    let g = Arc::clone(&a); // forth owner
    println!("Refence count: {}", Arc::strong_count(&f));

    let x = 10;
    let y = Arc::new(Mutex::new(x));
    println!("{}", *y.lock().unwrap());

    let number = Num{x: 32, y: 34};
    let num_ow = Arc::new(Mutex::new(number));
    println!("{:?}",(*num_ow.lock().unwrap()).x);
    println!("{:?}",(*num_ow.lock().unwrap()).y);
}