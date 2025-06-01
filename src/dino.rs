#![no_std]
#![no_main]

mod game;
mod rng;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::{self, Spawner};
use embassy_rp::{gpio::{Input, Pull}, i2c::{Config, I2c}};
use embassy_time::Timer;
use panic_probe as _;
use ssd1306::{
    I2CDisplayInterface, mode::DisplayConfig, prelude::DisplayRotation, size::DisplaySize128x64,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Config with Fast Mode Plus.
    let mut config = Config::default();
    config.frequency = 1_000_000;

    // TODO: async :)
    let i2c = I2c::new_blocking(p.I2C1, p.PIN_19, p.PIN_18, config);

    // Screen
    let interface = I2CDisplayInterface::new(i2c);
    let mut screen = ssd1306::Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // Init screen.
    screen.init().expect("failed screen init");
    screen.flush().expect("failed screen flush");

    let mut tick: u64 = 0;
    let mut rng = rng::LFSR::new(2025);
    let mut game = game::Game::new(&mut rng);

    let jump = Input::new(p.PIN_22, Pull::Up);

    loop {
        if jump.is_low() {
            game.jump();
        }

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
