use crate::rng::LFSR;
use defmt::{info};

const SCREEN_HEIGHT: usize = 64;

// (6, 10) in
// https://www.reddit.com/r/PixelArt/comments/kzqite/oc_cute_8x8_pixel_art_with_max_3_colours_per/#lightbox
// This is already left shifted because we know that this tile will appear on the left of the screen.
const DINO_STILL_TILE: u128 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010100 << 64;
const DINO_WALK_LEFT: u128 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010000 << 64;
const DINO_WALK_RIGHT: u128 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00000100 << 64;
const DINO_WIDTH: usize = 8;
const DINO_HEIGHT: usize = 8;

// Platform.
const GROUND_LEVEL: usize = SCREEN_HEIGHT - 3;

// Tiles for clouds.
const CLOUD_PART: [u128; 2] = [
    0b00000000000000000000000000000011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
];

pub struct Game<'a> {
    rng: &'a mut LFSR,

    gravel: u128,
}

impl<'a> Game<'a> {
    pub fn new(rng: &'a mut LFSR) -> Self {
        let gravel =
            rng.next_u32() as u128
            | ((rng.next_u32() as u128) << 32)
            | ((rng.next_u32() as u128) << 64)
            | ((rng.next_u32() as u128) << 96);

        Game {
            rng,
            gravel,
        }
    }

    pub fn next(&mut self, tick: &u64) -> [u128; SCREEN_HEIGHT] {
        let mut buffer = [0 as u128; SCREEN_HEIGHT];

        // Clouds
        let offset = tick / 3 % 128;
        for y in 0..CLOUD_PART.len() {
            buffer[y + 2] = (CLOUD_PART[y] << offset) | (CLOUD_PART[y] >> (127 - offset));
            buffer[y + 13] = ((CLOUD_PART[y] >> 64) << offset) | ((CLOUD_PART[y] >> 64) >> (127 - offset));
        }

        // Platform
        self.gravel = (self.gravel << 1) | (self.rng.next_bit() as u128);
        buffer[GROUND_LEVEL] = u128::MAX;
        buffer[GROUND_LEVEL + 1] = 0;
        buffer[GROUND_LEVEL + 2] = self.gravel;

        // Dino
        for y in 0..DINO_HEIGHT {
            let abs_y = GROUND_LEVEL - DINO_HEIGHT + y;

            // Animate walk with half the frequency of the ticks.
            let tile = if tick % 4 >= 2 {
                DINO_WALK_LEFT
            } else {
                DINO_WALK_RIGHT
            };

            buffer[abs_y] = buffer[abs_y] | ((tile << DINO_WIDTH * y) & (0xff << 120)) >> 2;
        }

        buffer
    }
}
