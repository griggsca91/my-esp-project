#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::gpio::{Io, Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::{
    clock::CpuClock,
    i2c::master::{Config as I2cConfig, I2c},
    time::Rate,
        delay::Delay,

};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use uln2003::{StepperMotor, ULN2003};


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    let mut motor = ULN2003::new(
        Output::new(peripherals.GPIO19, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO18, Level::Low, OutputConfig::default()),   
        Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO23, Level::Low, OutputConfig::default()),   
        Some(delay)
    );
    
    loop {
        motor.step().unwrap();
        blocking_delay(Duration::from_millis(1));
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples/src/bin
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

fn screen_main() -> ! {
    // generator version: 1.0.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO22) // change to your wiring
    .with_sda(peripherals.GPIO21); // change to your wiring;

    // SSD1306 driver in buffered graphics mode
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();


    let style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello Wazzup!", Point::new(0, 0), style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples/src/bin
}