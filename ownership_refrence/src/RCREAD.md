# What is the difference between Rc and Barrowing

| Feature     | `&T`               | `Rc<T>`                                 |
| ----------- | ------------------ | --------------------------------------- |
| Ownership   | Borrowed           | Shared (reference counted)              |
| Lifetime    | Compile-time       | Runtime                                 |
| Thread-safe | Yes                | No (`Rc` is not `Send`/`Sync`)          |
| Mutability  | Controlled via `&` | Needs `RefCell` for interior mutability |
| Overhead    | None (zero-cost)   | Some (for reference counting)           |
| Use case    | Temporary access   | Multiple long-lived owners              |


# Refcell and Cell deals with the mutability of data at run time and similarly Rc is helful at run time.

**1. Need to consider the scope of the mut reference and imut reference to consider that the ownership rules are followed or not.**



| Pointer Type | Use Case                                                   |
| ------------ | ---------------------------------------------------------- |
| `Box<T>`     | Simple ownership, linear structure                         |
| `Rc<T>`      | Shared ownership, branching                                |
| `RefCell<T>` | Mutability with borrowing checks at runtime (advanced use) |


| What you use        | Result         | Why                                 |
| ------------------- | -------------- | ----------------------------------- |
| `Box<Node>`         | ✅ Works        | Owns heap data, fixed size          |
| `&Box<Node>`        | ❌ Error        | Borrow, needs lifetime              |
| `&Node`             | ❌ Error        | Borrow, needs lifetime              |
| `Rc<Node>`          | ✅ Works        | Shared ownership, reference counted |
| `Rc<RefCell<Node>>` | ✅ For mutation | Shared + mutable                    |
