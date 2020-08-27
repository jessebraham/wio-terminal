use atsamd_hal::clock::GenericClockController;
use atsamd_hal::delay::Delay;
use atsamd_hal::gpio::*;
use atsamd_hal::hal::spi;
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::{PadPin, SPIMaster7, Sercom7Pad1, Sercom7Pad2, Sercom7Pad3};
use atsamd_hal::target_device::{MCLK, SERCOM7};

use ili9341::{spi::SpiInterface, Ili9341, Orientation};

/// ILI9341 LCD display pins (uses `SERCOM7`)
pub struct Display {
    /// LCD MISO pin
    pub miso: Pb18<Input<Floating>>,

    /// LCD MOSI pin
    pub mosi: Pb19<Input<Floating>>,

    /// LCD SCK pin
    pub sck: Pb20<Input<Floating>>,

    /// LCD chip select pin
    pub cs: Pb21<Input<Floating>>,

    /// LCD data/command pin
    pub dc: Pc6<Input<Floating>>,

    /// LCD reset pin
    pub reset: Pc7<Input<Floating>>,

    /// LCD backlight pin
    pub backlight: Pc5<Input<Floating>>,
}

impl Display {
    /// Initialize the display and its corresponding SPI bus peripheral,
    /// returning the display driver struct for later interaction.
    ///
    /// Sets the initial orientation of the display (default is
    /// LandscapeFlipped) and enables the backlight.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom7: SERCOM7,
        mclk: &mut MCLK,
        port: &mut Port,
        delay: &mut Delay,
    ) -> Ili9341<
        SpiInterface<
            SPIMaster7<Sercom7Pad2<Pb18<PfD>>, Sercom7Pad3<Pb19<PfD>>, Sercom7Pad1<Pb20<PfD>>>,
            Pb21<Output<PushPull>>,
            Pc6<Output<PushPull>>,
        >,
        Pc7<Output<PushPull>>,
    > {
        // Initialize the SPI peripherial on the configured pins, using SERCOM7 and
        // running at 20MHz.
        let gclk0 = clocks.gclk0();
        let spi = SPIMaster7::new(
            &clocks.sercom7_core(&gclk0).unwrap(),
            20.mhz(),
            spi::Mode {
                phase: spi::Phase::CaptureOnFirstTransition,
                polarity: spi::Polarity::IdleLow,
            },
            sercom7,
            mclk,
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port),
            ),
        );

        // Configure the chip select, data/command, and reset pins as push-pull outputs.
        let cs = self.cs.into_push_pull_output(port);
        let dc = self.dc.into_push_pull_output(port);
        let reset = self.reset.into_push_pull_output(port);

        // Configure the backlight pin as a push-pull output.
        //   HIGH - backlight enabled
        //   LOW  - backlight disabled
        let mut backlight = self.backlight.into_push_pull_output(port);
        backlight.set_high().unwrap();

        // Create the ILI9341 driver, and set its default orientation.
        let mut ili9341 = Ili9341::new_spi(spi, cs, dc, reset, delay).unwrap();
        ili9341
            .set_orientation(Orientation::LandscapeFlipped)
            .unwrap();

        ili9341
    }
}
