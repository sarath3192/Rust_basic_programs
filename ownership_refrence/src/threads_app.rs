use std::thread;

fn test1()-> (){
    println!("Hello world")
}

fn main(){

let s = String::from("Hello world");

let handle = thread::spawn(test1);
let handle = thread::spawn(||println!("{}",s));
// let handle = thread::spawn(println!("Hello world")); // println! is not a function it is macro and it is exicuted immidiatedly and it returns ().
// println!("{:?}",s);
}