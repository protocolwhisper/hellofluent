#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate alloc;
extern crate fluentbase_sdk;

use alloc::string::String;
use fluentbase_sdk::{
    basic_entrypoint,
    derive::{router, signature},
    SharedAPI,
};

#[derive(Default)]
struct ROUTER;

pub trait RouterAPI {
    fn deploy(&self);
    fn greeting(&self) -> String;  // Removed message parameter
}

#[router(mode = "solidity")]
impl RouterAPI for ROUTER {
    fn deploy(&self) {
        // any custom deployment logic here
    }

    #[signature("function greeting() external returns (string)")]
    fn greeting(&self) -> String {
        "Hello".into()
    }
}

basic_entrypoint!(ROUTER);
