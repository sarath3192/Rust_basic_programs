1. The concepts of Ownership and borrowing are to handle the Heap memory memory effectively and also in the context of multi threaded programming.
2. Ownership:
             Each value in Rust has an owner.
             There can only be one owner at a time.
             When the owner goes out of scope, the value will be dropped.

3. The most common concepts in a programming;
        The memory must be requested from the memory allocator at runtime.
        We need a way of returning this memory to the allocator when we’re done with our String.

4. life times also play's a role in effectively handling the memory in rust.
5.  let s1 = String::from("hello"); Here the S1 holds three values 1. ptr 2. len 3. capacity. The s1 is stored in the stack and the string is stored in the heap.
6. let s2 = s1 (We are only coping the values of S1 on the stack is copied to the S2 on the stack).
7. If we copy the data on the heap also, if the data on the heap is very large then there will be runtime cost for doing this, which can create a performace loss.
8. This is known as a double free error and is one of the memory safety bugs we mentioned previously.
9. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
10. In rust there is no concept shallow copy and deep copy, but we call the ownership transfer as move.
11. If you wanted to do the deep copy of the heap and stack data we will use clone method.
12. Ownership and Functions: The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
13. Return Values and Scope: 
        While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.
14. But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called references.

References and Barrowing:

1. References Barrow checking is happening at compile time ( the owner ship and barrow checks are happends at compile time)
2. 

Heap memory allocation and Related problems:

Heap allocation in programming introduces several memory-related issues that can lead to inefficiencies, crashes, or security vulnerabilities. 
Here are the main problems:

1. Memory leaks:
                a. This occurs when we not free used memory. This increase the memory consumption over the time whcich lead to crash of the program.
                b. This is avoided in rust by scope, ownership and borrowing in rust.

                void leak() {
                        int* ptr = malloc(sizeof(int)); // Allocated but never freed
                }

                Solution: Solution: Always free allocated memory using free(ptr) (C) or use smart pointers in C++.

2. Dangling pointer:
                a. Happen when a pointer still references memory that has already been freed.
                b. Accessing this memory leads to undefined behavior (e.g., crashes, corrupted data).

                int* ptr = malloc(sizeof(int));
                free(ptr);
                *ptr = 10;  // Undefined behavior


                Solution: Set pointers to NULL after freeing (ptr = NULL;).

3. Fragmentation:
                a. External Fragmentation: Small free memory blocks are scattered, preventing large allocations.
                b. Internal Fragmentation: Allocated memory is larger than required, leading to waste.
                c. Common in long-running programs (e.g., databases, OS kernels).
                
                Solution: Use memory pools or compacting garbage collectors (in managed languages like Java, Rust).

4. Double Free: 
                a. Happens when memory is freed twice, causing corruption in the heap.
                
                int* ptr = malloc(10);
                free(ptr);
                free(ptr);  // Undefined behavior

                Solution: Set ptr = NULL; after free(), so the second free() does nothing.

5. Buffer Overflows:
                a. Writing beyond allocated memory can overwrite critical data, leading to crashes or security vulnerabilities (e.g., hacking via stack smashing).
                
                char *buffer = malloc(10);
                strcpy(buffer, "This is too long!"); // Overflows the buffer

                Solution: Use bounds-checked functions like strncpy(), dynamic sizing, or safe languages (Rust).

6. Use-After-Free: 
                a. Accessing memory after it has been freed can lead to crashes, corruption, or security exploits.

                int* ptr = malloc(sizeof(int));
                free(ptr);
                printf("%d", *ptr);  // Undefined behavior

                Solution: Avoid accessing freed memory and use tools like Valgrind or AddressSanitizer to detect issues.

7. Out-of-Memory (OOM) Errors:
                a. Occurs when a program continuously allocates memory without releasing it, exhausting system memory.

                while (1) {
                        malloc(1000000);  // Causes system slowdown and crash
                }

                Solution: Check for failed allocations (if (ptr == NULL)) and limit memory usage.

How Different Languages Handle These Issues: 

Issue	           C/C++	        Rust	                Java/Python

Memory Leaks	  | Manual free()	| Ownership system   |	Garbage Collection (GC)
Dangling Pointers | Common Not possible | (borrow checker)   |	Not possible
Fragmentation	  | Possible	        | Less frequent	     |  GC compacts memory
Double Free	  | Undefined behavior	| Compile-time error |	GC prevents it
Buffer Overflow	  | Frequent	        | Checked with bounds|	Checked
Use-After-Free	  | Common              | Not possible	     |  GC prevents it
OOM Errors	  | Common              | Can happen	     |  Managed by GC






