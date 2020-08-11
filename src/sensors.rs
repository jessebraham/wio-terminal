use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::{Floating, Input, Pa12, Pa13, PfC, Port};
use atsamd_hal::sercom::{I2CMaster2, PadPin, Sercom2Pad0, Sercom2Pad1};
use atsamd_hal::target_device::{MCLK, SERCOM2};
use atsamd_hal::time::KiloHertz;

pub use lis3dh::accelerometer;
use lis3dh::{Lis3dh, SlaveAddr};

/// I2C Accelerometer pins (uses `SERCOM2`)
pub struct Accelerometer {
    /// `I2C0` bus clock pin
    pub scl: Pa12<Input<Floating>>,

    /// `I2C0` bus data pin
    pub sda: Pa13<Input<Floating>>,
}

impl Accelerometer {
    /// Initialize the LIS3DH accelerometer using the correct pins and
    // peripherals. Use the driver's default settings.
    pub fn new(
        clocks: &mut GenericClockController,
        sercom2: SERCOM2,
        mclk: &mut MCLK,
        sda: Pa13<Input<Floating>>,
        scl: Pa12<Input<Floating>>,
        port: &mut Port,
    ) -> Lis3dh<I2CMaster2<Sercom2Pad1<Pa13<PfC>>, Sercom2Pad0<Pa12<PfC>>>> {
        // The accelerometer is connected to the Wio Terminal's `I2C0` bus, so
        // based on the possible padouts listed in the datasheet it must use
        // `SERCOM2` and in turn `I2CMaster2`.
        let gclk0 = clocks.gclk0();
        let i2c = I2CMaster2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            KiloHertz(400),
            sercom2,
            mclk,
            sda.into_pad(port),
            scl.into_pad(port),
        );

        // The schematic states that the alternate I2C address `0x19` is used.
        Lis3dh::new(i2c, SlaveAddr::Alternate).expect("unable to initialize lis3dh")
    }
}
