fn main(){
    // a_on_stack is allocated memory stack on ram when the program is loaded from Rom to ram
    let a_on_stack = 10;
    // Similar a_on_stack, the b_on_stack and c_on stack also allocated memory on the stack memory of ram when program loaded
    // How does the compiler knows where to place the variable based on the type.
    // The types whos size is know at compile time is stored at stack and types whos size is not deterministic at compile time are stored on the heap.
    // i32, u32, ect are known sizes and where as box,vec sizes are not known at compile time so they are allocated on the heap.
    // But even we see that size of heap allocation is not known at compile time. Creating a pointer to store the allocated address of the heap allocation
    // which is know at compile time and fixed. The memory layout of all types in rust or infact c/C++ are well known at compile time. But the memory allocated
    // The amount of data stored on the heap is not known. Where the compiler just assigns the key in the stack and data in the heap.
    // As the where the memory allocated and how much allocated is found based heap allocation. This part is completely decide at run time.
    let b_on_stack = 20;
    let c_on_stack = a_on_stack;
    println!("a={},b={},c={}", a_on_stack,b_on_stack,c_on_stack);
}  