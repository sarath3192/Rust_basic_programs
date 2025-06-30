use std::io;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Data{
    rool_num: i32,
    height: i32,
}

 // From below we can clearly see we can modify both fields in the struct c at compile time and run time. But we can't make some fields mut or unmut.
// In a struct I wanted make some field mut and some immutable
// i.e
#[cfg(disabled)]
struct Data1{
      mut id_num: i32,
      salary: mut u32,
} 

// Note: why Refcell and cell are use we can use mut in side the struct also right.
// Field level mutation if we wanted to do use cell and refcell i.e:
#[derive(Debug)]
struct Data1 {
    id_num: i32,
    salary: Cell<u32>, // allows mutation even if `Data1` is not mutable
}
#[derive(Debug)]
struct Data2{
    id: i32,
    salary: RefCell<i32>,
}

fn main(){
    let a: i32 = 23;
    let _b = a;
    //Here the entire struct data can be mut or imut, but not some mut and some imute.
    let mut c = Data{rool_num: 12, height: 34};
    println!("{}{:?}",a, c);
    // These two below line work without error if the c is declared as mut.
    c.rool_num = 30;
    c.height = 40;
    println!("{:?}",c);
    // In the above the data is chaning at compile time if you wanted to change a value at run time how will we do it.
    let mut d: String = String::new();
    io::stdin().read_line(&mut d).expect("Error occured which taking assigning value");
    c.rool_num = d.trim().parse().expect("not able to convert");
    println!("{:?}",c);
    let k = Data1{id_num: 54, salary: Cell::new(32)};
    let m = Data2{id: 65, salary: RefCell::new(64)};
    k.salary.set(20);
    *m.salary.borrow_mut() = 10;
    println!("{:?}{:?}",k,m);
}


