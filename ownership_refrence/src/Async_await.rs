use std::thread;
extern crate libc;
use libc::{getpid, syscall, SYS_gettid};
fn main(){
   let handle = thread::spawn(move || {
    let pid = unsafe{ getpid()};
    let tid = unsafe{ syscall(SYS_gettid)   };

    println!("PID: {}", pid);
    println!("TID: {}", tid);
   });
   handle.join().unwrap();
}