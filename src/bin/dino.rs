#![no_std]
#![no_main]

use defmt::{info};
use defmt_rtt as _;
use embassy_rp::i2c::{I2c, Config};
use embassy_executor::{self, Spawner};
use embassy_time::Timer;
use ssd1306::{mode::DisplayConfig, prelude::DisplayRotation, size::{DisplaySize128x64}, I2CDisplayInterface};
use {panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Config is 
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

    // (6, 10) in https://www.reddit.com/r/PixelArt/comments/kzqite/oc_cute_8x8_pixel_art_with_max_3_colours_per/#lightbox
    let dino: [bool; 64] = [
        false, true,  true, true,  true, false, false, false,
        true,  false, true, false, true, false, false, false,
        true,  true,  true, true,  true, false, false, false,
        false, false, true, true,  true, false, false, false,
        false, true,  true, true,  true, false, true,  false,
        false, false, true, true,  true, false, true,  false,
        false, false, true, true,  true, true,  false, false,
        false, false, true, false, true, false, false, false,
    ];

    screen.clear_buffer();

    screen.set_pixel(0, 0, true);
    screen.set_pixel(1, 1, true);

    for y in 0..8 {
        for x in 0..8 {
            screen.set_pixel(x + 10, y + 10, dino[(y*8 + (7 - x)) as usize]);
        }
    }
    screen.flush().expect("failed screen flush");

    // Test.
    let mut base_y = 0;
    loop {

        base_y += 1;
        if base_y > 64 {
            base_y = 0;
        }
    }
}
