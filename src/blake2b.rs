include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;
use std::slice;

pub fn blake2b512_digest(input: &[u8]) -> &[u8] {
    unsafe {
        let len = input.len();
        let input = input.as_ptr();
        let h = hash(input, len as i32);
        slice::from_raw_parts(h, 64)
    }

}

// fn main() {
//     println!("Hello from Rust.");
//     unsafe {
//         call_hello_from_c();
//     }
// }