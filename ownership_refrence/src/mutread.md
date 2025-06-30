# In single threaded application like in cases where you are not sure mut or immut data field. Cases where you can't decide at compile time. There the RefCell and Cell comes handy.

# Mutable requirement in rust inside struct:

    How a struct is defined in general:
    struct Player_score{
        Name: String,
        Score: i32,
    }

    Binding = Giving a name to a value (let x = 5;), mut will tell the value is change ble or not.
    The mutable binding is not allowed inside the struct. But interior mutability is possible with Cell and Refcell
     
    Rust separates:

        Ownership → who owns the value
        Borrowing → who can access the value
        Mutability → who can change the value

        This strict separation helps prevent data races, bugs, and undefined behavior, especially in multi-threaded code.
    
# In single threaded there is not need for barrow checking as there is no data race ( Accessing same mememory, hardware at the same time, )

| Why runtime borrow checking matters (even single-threaded) |
| ---------------------------------------------------------- |
| Prevents logical bugs due to overlapping read/write        |
| Enforces **exclusive access** to mutable data              |
| Keeps behavior **predictable and safe**                    |
| Avoids undefined behavior in complex programs              |
| Allows **interior mutability** safely                      |

    
