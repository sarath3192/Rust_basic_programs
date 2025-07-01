use std::rc::Rc;

fn main(){

    // # Here the ownership transfer is happend once it is done we can't use the same variable
    let a = "Hello".to_string();
    // let d = a;
    // println!("{}",a);
    // # How to create multiple owners using RC
    let d = Rc::new(a);
    let b = d.clone();
    println!("{}",Rc::strong_count(&d));
    let e = d.clone();
    println!("{}",Rc::strong_count(&d));
    println!("{}",Rc::strong_count(&d));
    println!("{} {}",b, e);

    let mut k = String::from("World");
    let l = &k;
    let m = &k;
    let n = &mut k;
    // let o = &mut k;
    println!("{}",n);
// let b = &a;
// let c = &c;
//     let rc_example = "Rc example".to_string();
//     {
//         let Rc_a: Rc<String> = Rc::new(rc_example);
//         println!()

//     }
}