//! Tracks the orientation of the device using the LIS3DH accelerometer, and
//! rotates the display accordingly.

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as hal;

use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;
use embedded_graphics::style::PrimitiveStyleBuilder;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::target_device::{CorePeripherals, Peripherals};
use hal::{entry, Pins, Sets};

use ili9341::Orientation as DisplayOrientation;
use lis3dh::accelerometer::Orientation as AccelOrientation;

const IMG_HEIGHT: u32 = 64;
const IMG_WIDTH: u32 = 86;

// Given the width and height of the display, return the coordinate which would
// center the image, based on its dimensions defined above.
fn center_image(width: usize, height: usize) -> (i32, i32) {
    (
        (width as i32 / 2) - (IMG_WIDTH as i32 / 2),
        (height as i32 / 2) - (IMG_HEIGHT as i32 / 2),
    )
}

// Given an accelerometer orientation, return the equivalent display
// orientation, if one exists.
fn convert_orientation(orientation: AccelOrientation) -> Option<DisplayOrientation> {
    match orientation {
        AccelOrientation::LandscapeUp => Some(DisplayOrientation::LandscapeFlipped),
        AccelOrientation::LandscapeDown => Some(DisplayOrientation::Landscape),
        AccelOrientation::PortraitUp => Some(DisplayOrientation::Portrait),
        AccelOrientation::PortraitDown => Some(DisplayOrientation::PortraitFlipped),
        _ => None,
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = Pins::new(peripherals.PORT);
    let mut sets: Sets = pins.split();

    // Initialize the LIS3DH accelerometer, and create the orientation tracker.
    // The calibration value for Tracker was obtained experimentally, as directed in
    // the documentation.
    let mut lis3dh = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    let mut tracker = Tracker::new(3700.0);

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen, load an image of Ferris from a RAW file, and draw it to the
    // screen.
    // By default, the display is in the LandscapeFlipped orientation.
    let mut display = sets.display.init(
        &mut clocks,
        peripherals.SERCOM7,
        &mut peripherals.MCLK,
        &mut sets.port,
        &mut delay,
    );

    // The display's resolution is 320x240. I'm too lazy to deal with orientation
    // for something as trivial as a backdrop, so it's larger than the display in
    // one dimension.
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let backdrop = Rectangle::new(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
    backdrop.draw(&mut display).unwrap();

    // Load the RAW image file into a renderable format. Determine the coordinate to
    // center the image on the display, and draw the image there.
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(
        include_bytes!("../assets/ferris.raw"),
        IMG_WIDTH,
        IMG_HEIGHT,
    );
    let (x, y) = center_image(display.width(), display.height());
    let image: Image<_, Rgb565> = Image::new(&image_raw, Point::new(x, y));
    image.draw(&mut display).unwrap();

    // Determine a baseline orientation. This doesn't really matter initially, but
    // will be updated periodically.
    let acceleration = lis3dh.accel_raw().unwrap();
    let mut prev_orientation = tracker.update(acceleration);

    loop {
        // Get the current orientation of the device.
        let acceleration = lis3dh.accel_raw().unwrap();
        let orientation = tracker.update(acceleration);

        // Attempt to convert the accelerometer orientation to a display orientation. If
        // successful, rotate the display.
        let disp_orientation = convert_orientation(orientation);
        if disp_orientation.is_some() && orientation != prev_orientation {
            let disp_orientation = disp_orientation.unwrap();
            display.set_orientation(disp_orientation).unwrap();

            let (x, y) = center_image(display.width(), display.height());
            let image: Image<_, Rgb565> = Image::new(&image_raw, Point::new(x, y));

            backdrop.draw(&mut display).unwrap();
            image.draw(&mut display).unwrap();

            prev_orientation = orientation;
        }
    }
}
