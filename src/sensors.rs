use atsamd_hal as hal;
use hal::clock::GenericClockController;
use hal::gpio::{Floating, Input, Pa12, Pa13, PfC, Port};
use hal::sercom::{I2CMaster2, PadPin, Sercom2Pad0, Sercom2Pad1};
use hal::target_device::{MCLK, SERCOM2};
use hal::time::KiloHertz;

use lis3dh::{Lis3dh, SlaveAddr};

/// Convenience function for setting up the accelerometer's labelled `SDA` and
/// `SCL` pins to operate as an I2C master running at the desired frequency.
pub fn accelerometer(
    clocks: &mut GenericClockController,
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sda: Pa13<Input<Floating>>,
    scl: Pa12<Input<Floating>>,
    port: &mut Port,
) -> Lis3dh<I2CMaster2<Sercom2Pad1<Pa13<PfC>>, Sercom2Pad0<Pa12<PfC>>>> {
    let gclk0 = clocks.gclk0();
    let i2c = I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        KiloHertz(400),
        sercom2,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    );

    Lis3dh::new(i2c, SlaveAddr::Alternate).unwrap()
}
