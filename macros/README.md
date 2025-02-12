Links:

1. https://doc.rust-lang.org/reference/macros-by-example.html
2. 

Attributes and Macros:

1. In Rust, attributes are metadata applied to code elements like functions, structs, or modules. Some attributes serve both conditional compilation and macro creation purposes.
3. There is difference between attributes and Macros in rust.
4. Attribute types are Normal attribute and attribute macros.
5. #[derive()] is procedural macro and #[automatically_generated] is attribute.
6. Attributes Helps tools like rustfmt and clippy to recognize auto-generated code.
7. Attribute macros are defined in the standard library.
8. Custom macro used to add specified code with enums and structs.
   i.e #[derive()] 
9. Attribute like macros that can be used any time.
10. Function like macros.
11. Normal or simple Attributes like #[inline] or #[cfg] provides metadata for the compiler and where as #[derive()] expands to code.
12. The macros which look like function i.e: println!, vec! and format!
13. Differences Between macro_rules! and Procedural Macros.
14. There are two types: a) Declarative Macros and b) Procedural macros.
15. Types of Procedural Macros:
    a)  Function-like Macros (#[proc_macro])
    b)  Derive Macros (#[proc_macro_derive]) and
    c)  Attribute Macros (#[proc_macro_attribute]).
16. Procedural macros in Rust allow you to generate or modify code at compile time by manipulating the Abstract Syntax Tree (AST).
17. Procedural macros needs external "proc_macro" crate.
18. Practice more that will understand the difference.
19. #[macro_export] is a metadata attribute used by the compiler in rust.
// ✅ It provides extra information to the Rust compiler about how a macro should be handled.
// ✅ It does not execute code but modifies how macros are made available across modules and 

20. let k = { let a = 3;
                a + b
                };
21. 