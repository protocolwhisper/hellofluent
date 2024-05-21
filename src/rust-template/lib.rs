#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
extern crate alloc;
extern crate fluentbase_sdk;

use alloc::vec::Vec;
use fluentbase_sdk::{LowLevelAPI, LowLevelSDK};

#[cfg(feature = "cairo")]
mod cairo;
#[cfg(feature = "contract_input_check_recode")]
mod contract_input_check_recode;
// #[cfg(feature = "erc20")]
// mod erc20;
#[cfg(feature = "evm_call_from_wasm")]
mod evm_call_from_wasm;
#[cfg(feature = "greeting")]
mod greeting;
#[cfg(feature = "stack")]
mod stack;

macro_rules! export_and_forward {
    ($fn_name:ident) => {
        #[cfg(not(feature = "std"))]
        #[no_mangle]
        #[cfg(target_arch = "wasm32")]
        pub extern "C" fn $fn_name() {
            #[cfg(feature = "greeting")]
            greeting::$fn_name();
        }
    };
}

export_and_forward!(deploy);
export_and_forward!(main);

pub(crate) fn get_input() -> Vec<u8> {
    let input_size = LowLevelSDK::sys_input_size();
    let mut input_buffer = Vec::with_capacity(input_size as usize);
    LowLevelSDK::sys_read(&mut input_buffer, 0);
    input_buffer
}

pub(crate) fn write_output(output: Vec<u8>) {
    LowLevelSDK::sys_write(&output);
    LowLevelSDK::sys_halt(0);
}