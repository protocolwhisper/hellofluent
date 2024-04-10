#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
extern crate alloc;
extern crate fluentbase_sdk;

use fluentbase_sdk::{LowLevelAPI, LowLevelSDK};


#[cfg(feature = "greeting")]
mod greeting;

macro_rules! export_and_forward {
    ($fn_name:ident) => {
        #[cfg(not(feature = "std"))]
        #[no_mangle]
        #[cfg(target_arch = "wasm32")]
        pub extern "C" fn $fn_name() {
            #[cfg(feature = "greeting")]
            greeting::$fn_name();
            #[cfg(feature = "panic")]
            panic::$fn_name();
            #[cfg(feature = "rwasm")]
            rwasm::$fn_name();
            #[cfg(feature = "stack")]
            stack::$fn_name();
        }
    };
}

export_and_forward!(deploy);
export_and_forward!(main);

pub(crate) fn deploy_internal<const N: usize>(bytes: &'static [u8; N]) {
    LowLevelSDK::sys_write(bytes);
    LowLevelSDK::sys_halt(0);
}
