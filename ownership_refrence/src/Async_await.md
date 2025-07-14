# Async and Await:
    In general there is thread programming, as these threads act as seperate processes in linux. which are created by thread::spwan. They will have process Ids. These processes are handled by the linux kernel.

# 🧵 Thread vs ⚙️ Async/Await (in Rust or any language):

| Feature               | **Thread**                                | **Async/Await**                                 |
| --------------------- | ----------------------------------------- | ----------------------------------------------- |
| **Concurrency type**  | OS-level (via threads)                    | Cooperative (via event loop)                    |
| **Execution**         | True parallelism on multiple CPU cores    | Single-threaded or multi-threaded, non-blocking |
| **Context switching** | Managed by OS (heavy, slower)             | Lightweight (faster)                            |
| **Memory usage**      | Higher (each thread has its own stack)    | Lower (uses a single stack and `Future`s)       |
| **Best for**          | CPU-bound tasks (e.g., heavy computation) | IO-bound tasks (e.g., networking, file IO)      |
| **Blocking**          | Can block the thread                      | Uses `await` to yield control without blocking  |
| **Error handling**    | Standard Result/unwrap()                  | Uses `Result`, `.await`, and `.await?`          |


# Difference between Thread and Async

| Feature            | `thread::spawn`     | `tokio::spawn` / `async`             |
| ------------------ | ------------------- | ------------------------------------ |
| Creates OS thread? | ✅ Yes               | ❌ No                                 |
| Gets new PID?      | ❌ No (same process) | ❌ No                                 |
| Gets new TID?      | ✅ Yes               | ❌ No (unless multi-threaded runtime) |
