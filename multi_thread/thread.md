# spawn: 

    fn test1()->(){
        println!("inside a thread")
    }
    thread::spawn(test1);

    Note:
    1. test1 is a function pointer, 
    2. Which automatically implements FnOnce + send + 'static.
    3. If we pass a test1() call it returns () tuple, which does not implement FnOnce +     send + 'static. Which will raise a error.

🔁 Trait Hierarchy:
    
    Fn < FnMut < FnOnce

    If a closure implements Fn, it also implements FnMut and FnOnce.
    If it moves captured variables, it may only implement FnOnce

🏷️ What is a Marker Trait in Rust?

    A marker trait is a trait that has no methods or associated items.
    It's used to mark or label types with certain properties, often for compile-time checks.