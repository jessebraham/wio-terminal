//! `wio_terminal` is a board support package which provides a type-safe API
//! for the Seeed Studio [Wio Terminal].
//!
//! This crate is essentially a thin wrapper for [atsamd-hal], and re-exports
//! many of its members.
//!
//! [Wio Terminal]: https://www.seeedstudio.com/Wio-Terminal-p-4509.html
//! [atsamd-hal]: https://github.com/atsamd-rs/atsamd
//!

#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal;
pub use atsamd_hal::common::*;
pub use atsamd_hal::samd51::*;
pub use atsamd_hal::target_device;

pub mod pins;
pub use pins::{Pins, Sets};

pub mod sensors;
