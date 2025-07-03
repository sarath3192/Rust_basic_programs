# Declaration:
    1. Creation of a prototype of variable, struct or function is termed as Declaration 
    2. At the time of Declaration it is just memory layout with out actual values.

    i.e:
        let a: i32;  // This is not allowed in rust, but this is how a variable declaration looks.

        struct Data{ // This is allowed and is a stuct declaration in rust
            num: i32,
            name: String, 
        }

        fn sum(a: i32, b: i32)-> i32; // This is also function prototype or a function declaration, This is not allowed out side the main but this is
                                      // valid inside a trait.

        trait car{
            fn speed()-> i32;
            fn wheels()-> u32;
            fn model() -> str;
        }

# Definition: 
    When the definition present then memory is allocated for the type. The created data type need to be initialized.

    i.e:
    let a: i32 = 6; // Definition and memory allocation happends. Infact it not done at compile time the allocation done at run time in stack or heap(in ram).

    struct Data{ 
        Name: String
    }
    fn main(){
        let a = Data{Name: "Hello".to_string()}; // Here the memory is allocated in stack and heap. So it is a Definition.
    }

    fn speed()->i32{ // This the definition of the function.
        10
    }

# Why clone has a performance cost.

    Summary:
        Copy = fast, stack-only, simple types ✅
        Clone = can be expensive, especially with heap data ❌