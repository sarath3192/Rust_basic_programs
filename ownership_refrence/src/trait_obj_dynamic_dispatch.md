
| Aspect      | Static Dispatch                      | Dynamic Dispatch                       |
| ----------- | ------------------------------------ | -------------------------------------- |
| Syntax      | `impl Trait` or concrete types       | `&dyn Trait` or `Box<dyn Trait>`       |
| Resolved At | Compile time                         | Runtime                                |
| Performance | Faster (no runtime lookup)           | Slightly slower (due to vtable lookup) |
| Flexibility | Less (type must be known at compile) | More (type can change at runtime)      |
| Use Case    | High performance, known types        | Plugins, runtime polymorphism          |


1. You can write code that works with any type implementing a trait, even if the exact type is unknown at compile time.
2. Great for plugin systems or heterogeneous collections (Vec<Box<dyn Trait>>).