//! Blink the user LED.

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{entry, Pins};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut pins = Pins::new(peripherals.PORT);
    let mut user_led = pins.user_led.into_open_drain_output(&mut pins.port);

    loop {
        user_led.set_high().unwrap();
        delay.delay_ms(200u8);

        user_led.set_low().unwrap();
        delay.delay_ms(200u8);
    }
}
