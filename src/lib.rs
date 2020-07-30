#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use atsamd_hal as hal;
pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::target_device as pac;

pub mod pins;
pub use pins::Pins;
