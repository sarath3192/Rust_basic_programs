use std::any::type_name;

// fn print_type_of<T>(_: T) {
//     println!("{}", type_name::<T>());
// }

fn main(){
    // let a: Box<i32> = Box::new(10);
    // // print_type_of(a.clone());                // The scope of the taking owner ship is limited to the function only.
    // let b: Box<i32> = a.clone();
    // println!("{}{}", a,b);  
    // let c: Box<i32> = Rc::new(Box::new(20));
    // let d: B0x<i32> = c.clone(c);
    // println("{}{}", c,d);

    let mut a: Box<i32> = Box::new(10);
    let mut c: Box<i32> =  Box::new(20);
    println!("{}",a);
    // let b = &mut a;
    // // b.push(20));
    // // b = &mut c;
    // println!("{}",b);
    // // let c = &mut a;
    // // print_type_of(b);
    // // println!("{}{}",b, c);
}

/* Definition = Declaration + Allocation (or Implementation)
// **Declaration examples:**
#[derive(Debug)] 
struct Person{  // This is a struct declaration, memory is not allocated while declaration of a struct.
    name: String,
    id_no: i32,
}
#[derive(Debug)]
struct Model{  // This is also struct declaration, Memory is not allocated while declaring a structure.
    input_layer_nodes: u32,
    middel_layer_nodes: u32,
    output_layer_nodes: u32,
}
fn main(){

    //The Model is defined, the memory for it is allocated on the stack and assigned to a. Which points to the instance of model.
    //In 
    let a = Model{input_layer_nodes: 10, middel_layer_nodes: 10, output_layer_nodes: 10};

     //1. The memory for the "Hello" String is allocated on the heap, the address is stored on the stack for access.
    let a = Person{name: String::new("Hello"), id_no: 1234_1234_1234_1234};
    println!("{:?}", a);
}*/
