//! Low level access to MSP430 microcontrollers
//!
//! This crate is based on [cortex-m](https://docs.rs/cortex-m)
//! crate by Jorge Aparicio (@japaric).
//!
//! It provides:
//!
//! - Access to core registers like SR and SP.
//! - Interrupt manipulation mechanisms
//! - Safe wrappers around assembly instructions like `nop`

#![deny(missing_docs)]
#![feature(llvm_asm)]
#![no_std]

extern crate bare_metal;

#[macro_use]
pub mod asm;
pub mod interrupt;
pub mod peripheral;
pub mod register;
