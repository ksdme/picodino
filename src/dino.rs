#![no_std]
#![no_main]

mod background;

use defmt::{info};
use defmt_rtt as _;
use embassy_rp::i2c::{Config, I2c};
use embassy_executor::{self, Spawner};
use embassy_time::Timer;
use ssd1306::{mode::DisplayConfig, prelude::DisplayRotation, size::{DisplaySize128x64}, I2CDisplayInterface};
use {panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Config with Fast Mode Plus.
    let mut config = Config::default();
    config.frequency = 1_000_000;

    // TODO: async :)
    let mut i2c = I2c::new_blocking(p.I2C1, p.PIN_19, p.PIN_18, config);

    // Screen
    let mut interface = I2CDisplayInterface::new(i2c);
    let mut screen = ssd1306::Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // Init screen.
    screen.init().expect("failed screen init");
    screen.flush().expect("failed screen flush");

    let mut bg = background::Background::default();

    loop {
        let pixels = bg.next();
        for y in 0..3 {
            for x in 0..128 {
                screen.set_pixel(x, 61 + y, pixels[y as usize * 128 + x as usize]);
            }
        }
        screen.flush().expect("could not flush");

        Timer::after_millis(33).await;
    }
}
