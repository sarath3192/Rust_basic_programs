fn main(){
    // In the below example even the a is camptured the ownership is not transfered
    // Because the closures implement Fn trait in the below case. which just creates a reference of a.
    let a = String::from("hello");
    let mut b = 10;
    let c = 30;
    let x = || {
                // let b = a; moves a into the closure (because String is not Copy).
                // So Rust infers that the closure must take a by move, even though you didn't write move
                // As a result, the closure becomes FnOnce and owns a.
                println!("{}",c);
                b += 10;
                let d = a; // 👈 `a` is moved into `b`
                println!("{} {} {}",d, b, c);
            }; 
    x();
    // println!("{}",a);
    println!("{}",c);
    println!("{}",b);


    // let mut b = 20;
    // let x  = 
}