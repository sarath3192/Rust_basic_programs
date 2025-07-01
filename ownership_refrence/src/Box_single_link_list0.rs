use std::{io, boxed::Box};
use io::Error;
#[derive(Debug)]
struct SingleData{
    number: i32,
    reference: Option<Box<SingleData>>,
}
fn add_node()-> Result<SingleData, Error> {
    let mut input = String::new();
    println!("Please enter number");

    io::stdin()
    .read_line(&mut input)
    .expect("Error while reading");

    let b: i32 = input.trim().parse().expect("Not a valid number");
    let first =  SingleData{number: b, reference: None};
    let second = SingleData{number: b+1, reference: Some(Box::new(first))};
    let third = SingleData{number: b+2, reference: Some(Box::new(second))};
    Ok(third)
}


fn main(){
   let a = add_node().unwrap();
}