#![no_std]
#![no_main]

mod game;

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

    let game = game::Game::default();
    let mut tick: u64 = 0;

    loop {
        tick = tick.wrapping_add(1);

        let buffer = game.next(&tick);
        // TODO: screen.draw() annoying does &[u8], but, we do row wise internal
        // buffer. So, figure out a way to be more efficient here.
        for y in 0..64 {
            for x in 0..128 {
                screen.set_pixel(x, y, ((buffer[y as usize] >> (127 - x)) & 1) == 1);
            }
        }
        screen.flush().expect("could not flush");

        Timer::after_millis(33).await;
    }
}
