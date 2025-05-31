#![no_std]
#![no_main]

use embassy_rp::i2c::{self, I2c};
use {panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    loop {}
}

