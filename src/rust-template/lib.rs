#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate fluentbase_sdk;
use fluentbase_sdk::{basic_entrypoint, SharedAPI};

#[derive(Default)]
struct GREETING;

impl GREETING {
    fn deploy<SDK: SharedAPI>(&self) {
        // any custom deployment logic here
    }

    fn main<SDK: SharedAPI>(&self) {
        // write "Hello, World" message into output
        const HELLO: &[u8] = b"Hello, world";
        SDK::write(HELLO.as_ptr(), HELLO.len() as u32);
    }
}

basic_entrypoint!(GREETING);
