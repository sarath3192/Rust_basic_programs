// How to create a vec type in rust.
// Syntax of a variable: let variable_name : type of the variable = value assigned.
// use std::vec;
// fn main(){
//     let mut  a: Vec<i32> = Vec::new();
//     a.push(10);
//     a.push(20);
//     a.push(30);
//     let c: Option<i32> = a.pop(); // if you wated to get the last pushed value into the vec
//     let d: Option<&i32> = a.get(1); // get helps the any value with index parameter.
//     match c {
//         Some(x) => println!("{}",x),
//         None => println!("no value is returned"),
//     };
//     let mut n =  &mut a;
//     n.push(10);
//     let mut m = &mut a;
//     println!("{:?}",n);
//     let mut l = &mut a;
//     println!("{:?}",l);
//     println!("{}",a.len());
//     let mut b = vec![0,2,4,5,7];
//     // println!("{}",a[0]);
//     println!("{}",b[2]);
//     println!("{}",b[4]);
// }

use std::sync::MutexGuard;

// Ownership explained
// 1. Each value in rust have a owner
// 2. There can be one owner at time
// 3. The value is dropped if the value is goes out of scope. 
fn main() {
    let a =  32;
    let a: [i32; 5] = [1,3,4,5,5];
    let mut  x: (i32,u32,i32,&str) = (1,3,4,"hello");
    x.0 = 32;
    println!("{:?}",x);
    println!("{:?}",a);
}