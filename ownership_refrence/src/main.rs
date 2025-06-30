//SCOPE:
/*fn main() {
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
*/

//HOW MEMORY ALLOCATED ON THE STACK AND HEAD ARE DIFFERENT
/*fn main() {
    let x = 5; // The memory is created on the stack
    let y = x; // The x is coppied to y where y will have different memory as we know the size of the x and y compile time.
               // In case of stack doing a deep copy is easy and feasible as we know the size at compile time.
}
*/

//THINK AND UNDERSTAND HOW THE MEMORY ALLOCATION HAPPENDS IN THE BELOW CASE:
/*fn main() {
    let s1 = String::from("hello"); // Here the memory is allocated on the Heap for the "hello" and its pointer is stored on the s1 which is on the stack.
     let s2 = s1;
}*/

// OWNERSHIP IN ACTION
/* fn main(){
    let s1 = String::from("hello"); // s1 comes into scope ownership is created for the s1
    let s2 = s1;                    // s1 loses owner ship and s2 gets the ownership of the string

    println!("{s1}, world!");               // So here the s1 is no more available.
}*/

//Scope and droping the memory:
/*fn main(){
    let mut s = String::from("hello"); // This holding the value "hello" 
    s = String::from("ahoy");          // This is now holding the value ahoy once this is allocated the the hello is freed before assigning the ahoy to s

    println!("{s}, world!");
}*/

// Stack-Only Data: Copy

let x = 5;
let y = x;

println!("x = {x}, y = {y}");

// Ownership and Functions:

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// Return Values and Scope.

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
