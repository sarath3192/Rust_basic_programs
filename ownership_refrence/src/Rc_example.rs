use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct Values{
    n: RefCell<i32>,
    m: i32,
}

fn main(){
    let data = Rc::new(RefCell::new(10));

    let a = Rc::clone(&data);
    let b = Rc::clone(&data);
    let c = data.clone();
    let d = data.clone();

    *a.borrow_mut() = 10;
    *b.borrow_mut() = 20;

    println!("{}", data.borrow());
    println!("{:?}", a);

    let k = RefCell::new(Values{n: RefCell::new(10) , m: 20});
    let l = RefCell::new(60);
    *k.borrow_mut() = Values{
        n: l,
        m: 40,
    };
    k.borrow().n.replace(65);
    println!("{:?}",k);
    println!("Rc: {:?}", k.borrow());

    let x = Mutex::new(10);
    *x.lock().unwrap() += 5;
    println!("Mutex: {}", x.lock().unwrap()); 
    // *k.borrow_mut() = 10;
    // *k.borrow_mut() = 20;
}