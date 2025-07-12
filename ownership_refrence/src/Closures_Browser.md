# Closures:
    
    In Rust are anonymous functions that can capture variables from their surrounding scope. 
    They're very powerful and flexible.

# Examples:

    🔍 How Closures Work (Internally):

        Closures are compiled into structs that:
        May capture variables from the environment.

    Implement one or more of these traits:

            Fn → captures by reference
            FnMut → captures by mutable reference
            FnOnce → captures by value (ownership)

        1.  let x = 5;
            let print = || println!("{}", x); // borrows x
            print();

        2. let mut x = 5;
           let mut change = || x += 1; // borrows x mutably
           change();
           println!("{}", x); // 6

        3. let s = String::from("hello");
           let consume = move || println!("{}", s); // takes ownership of s
           consume();
           // println!("{}", s); // ❌ Error: s was moved
        
| Trait    | Capture Method            | When Implemented                          |
| -------- | ------------------------- | ----------------------------------------- |
| `Fn`     | By reference (`&T`)       | Closure only **reads** captured variables |
| `FnMut`  | By mutable ref (`&mut T`) | Closure **modifies** captured variables   |
| `FnOnce` | By value (`T`)            | Closure **takes ownership** of variables  |

            