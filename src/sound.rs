use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::{Floating, Input, Pd11, Port};
use atsamd_hal::prelude::*;
use atsamd_hal::pwm::{TCC0Pinout, Tcc0Pwm};
use atsamd_hal::target_device::{MCLK, TCC0};

/// Buzzer control pin
pub struct Buzzer {
    /// Buzzer control pin
    pub ctr: Pd11<Input<Floating>>,
}

impl Buzzer {
    /// Initialize the pin connected to the piezo buzzer. Configure the pin as a
    /// PWM output using TCC0, and return the Pwm instance.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        tcc0: TCC0,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> Tcc0Pwm {
        let pinout = TCC0Pinout::Pd11(self.ctr.into_function_f(port));

        let gclk0 = clocks.gclk0();
        let pwm0 = Tcc0Pwm::new(&clocks.tcc0_tcc1(&gclk0).unwrap(), 1.khz(), tcc0, pinout, mclk);

        pwm0
    }
}
