use std::thread;

fn test1()-> (){
    println!("Hello world")
}

fn main(){

let s = String::from("Hello world");

let handle1 = thread::spawn(test1);
let handle2 = thread::spawn(move||println!("{}",s));
handle1.join().unwrap();
handle2.join().unwrap();  
// let handle = thread::spawn(println!("Hello world")); // println! is not a function it is macro and it is exicuted immidiatedly and it returns ().
// println!("{:?}",s);
}