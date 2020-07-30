//! Wio Terminal pins

use super::hal;
use hal::clock::GenericClockController;
use hal::gpio::{self, *};
use hal::sercom::{PadPin, Sercom2Pad0, Sercom2Pad1, UART2};
use hal::target_device::{MCLK, SERCOM2};
use hal::time::Hertz;
use hal::{define_pins, target_device};

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their functions
    struct Pins,
    target_device: target_device,

    // The following pin names were adapted from the labels in the schematic.
    // They will likely evolve over time.
    // They're not in any particular order.

    /// USER_LED
    pin user_led = a15,

    /// BUTTONS
    pin button1 = c26,
    pin button2 = c27,
    pin button3 = c28,

    /// SWITCHES
    pin switch_x = d8,
    pin switch_y = d9,
    pin switch_z = d10,
    // pin switch_b = d12,  FIXME: why does this cause a build error?
    pin switch_u = d20,

    /// I2C
    pin i2c0_scl = a12,
    pin i2c0_sda = a13,
    pin i2c1_scl = a16,
    pin i2c1_sda = a17,

    /// SPI
    pin spi_miso = b0,
    pin spi_cs = b1,
    pin spi_mosi = b2,
    pin spi_sck = b3,

    /// UART
    pin txd = b26,
    pin rxd = b27,

    /// USB
    pin usb_dm = a24,
    pin usb_dp = a25,
    pin usb_host_en = a27,

    /// LCD
    pin lcd_miso = b18,
    pin lcd_mosi = b19,
    pin lcd_sck = b20,
    pin lcd_backlight = c5,
    pin lcd_dc = c6,
    pin lcd_reset = c7,
    pin lcd_xl = c10,
    pin lcd_yu = c11,
    pin lcd_xr = c12,
    pin lcd_yd = c13,

    /// GYROSCOPE
    pin gyroscope_int1 = c21,

    /// I2S
    pin i2s_lrclk = a20,
    pin i2s_sdin = a21,
    pin i2s_sdout = a22,
    pin i2s_blck = b16,

    /// BUZZER
    pin buzzer_ctr = d11,

    /// MICROPHONE
    pin mic_output = c30,

    /// MCU FLASH
    pin mcu_flash_qspi_io0 = a8,
    pin mcu_flash_qspi_io1 = a9,
    pin mcu_flash_qspi_io2 = a10,
    pin mcu_flash_qspi_io3 = a11,
    pin mcu_flash_qspi_clk = b10,
    pin mcu_flash_qspi_cs = b11,

    /// SD CARD
    pin sd_mosi = c16,
    pin sd_sck = c17,
    pin sd_miso = c18,
    pin sd_cs = c19,
    pin sd_det = d12,

    /// WIFI/BLE
    pin rtl8720d_chip_pu = a18,
    pin rtl8720d_hspi_mosi = b24,
    pin rtl8720d_hspi_clk = b25,
    pin rtl8720d_rxd = c22,
    pin rtl8720d_txd = c23,
    pin rtl8720d_hspi_miso = c24,
    pin rtl8720d_hspi_cs = c25,

    /// GPIO
    pin a0_d0 = b8,
    pin a1_d1 = b9,
    pin a2_d2 = a7,
    pin a3_d3 = b4,
    pin a4_d4 = b5,
    pin a5_d5 = b6,
    pin a6_d6 = a4,
    pin a7_d7 = b7,
    pin a8_d8 = a6,

    /// FPC
    pin fpc_d3_pwm3 = b28,
    pin fpc_d4_pwm4 = b17,
    pin fpc_d5_pwm5 = b29,
    pin fpc_d6_pwm6 = a14,
    pin fpc_d7_a7 = c1,
    pin fpc_d8_a8 = c2,
    pin fpc_d9_a9 = c3,
    pin fpc_d10_pwm10 = c4,
    pin fpc_d11_a11 = c31,
    pin fpc_d12_a12 = d0,
    pin fpc_d13_a13 = d1,

    /// DAC
    pin dac0 = a2,
    pin dac1 = a5,

    /// GPCLK
    pin gpclk0 = b14,
    pin gpclk1 = b12,
    pin gpclk2 = b13,

    /// SWD
    pin swdclk = a30,
    pin swdio = a31,

    /// XIN/XOUT
    pin xin = b22,
    pin xout = b23,

    /// MISCELLANEOUS
    pin sync = a19,
    pin swo = b30,
    pin ir_ctl = b31,
    pin output_ctr_5v = c14,
    pin output_ctr_3v3 = c15,
    pin irq0 = c20,
);

/// Sets of pins split apart by category
pub struct Sets {
    /// QSPI Flash pins
    pub flash: QSPIFlash,

    /// SD Card pins
    pub sd_card: SDCard,

    /// UART (external pinout) pins
    pub uart: UART,

    /// USB pins
    pub usb: USB,
}

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let flash = QSPIFlash {
            sck: self.mcu_flash_qspi_clk,
            cs: self.mcu_flash_qspi_cs,
            d0: self.mcu_flash_qspi_io0,
            d1: self.mcu_flash_qspi_io1,
            d2: self.mcu_flash_qspi_io2,
            d3: self.mcu_flash_qspi_io3,
        };

        let sd_card = SDCard {
            cs: self.sd_cs,
            mosi: self.sd_mosi,
            sck: self.sd_sck,
            miso: self.sd_miso,
            det: self.sd_det,
        };

        let uart = UART {
            rx: self.rxd,
            tx: self.txd,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        Sets {
            flash,
            sd_card,
            uart,
            usb,
        }
    }
}

/// QSPI Flash pins
pub struct QSPIFlash {
    pub sck: Pb10<Input<Floating>>,
    pub cs: Pb11<Input<Floating>>,
    pub d0: Pa8<Input<Floating>>,
    pub d1: Pa9<Input<Floating>>,
    pub d2: Pa10<Input<Floating>>,
    pub d3: Pa11<Input<Floating>>,
}

/// SD Card pins
pub struct SDCard {
    pub cs: Pc19<Input<Floating>>,
    pub mosi: Pc16<Input<Floating>>,
    pub sck: Pc17<Input<Floating>>,
    pub miso: Pc18<Input<Floating>>,
    pub det: Pd12<Input<Floating>>,
}

/// UART pins
pub struct UART {
    pub tx: Pb26<Input<Floating>>,
    pub rx: Pb27<Input<Floating>>,
}

impl UART {
    /// Convenience for setting up the labelled TXD, RXD pins to operate as a
    /// UART device at the specified baud rate.
    pub fn uart<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom2: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART2<Sercom2Pad1<gpio::Pb27<gpio::PfC>>, Sercom2Pad0<gpio::Pb26<gpio::PfC>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            baud.into(),
            sercom2,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    pub fn usb_allocator(
        self,
        usb: target_device::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UsbBusAllocator<UsbBus> {
        use target_device::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(
            usb_clock,
            mclk,
            self.dm.into_function(port),
            self.dp.into_function(port),
            usb,
        ))
    }
}
