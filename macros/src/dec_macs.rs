// Yes! #[macro_export] is a metadata attribute in Rust.
// Why is #[macro_export] Metadata?

// ✅ It provides extra information to the Rust compiler about how a macro should be handled.
// ✅ It does not execute code but modifies how macros are made available across modules and 

#[macro_export] 
macro_rules! say_hello {
    () => {
        println!("Hello world");
    };
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]  // Makes the macro available globally
macro_rules! say_hello2 {
    () => {
        println!("Hello from a macro!");
    };
}
// #[macro_export]
// macro_rules! check1{
//     ($($l:expr)) => {
//         $(
//         println!("{}",$l);
//         )
//     };
// }
pub fn add(a: u32, b: u32) {
    println!("{}{}",a,b);
}