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
const PLATFORM_PART: [u128; 3] = [
    u128::MAX,
    0,
    0b00110001010000101001100101011001011001111000000000000100111011100011000101000010100110010101100101100111100000000000010011101110,
];
const GROUND_LEVEL: usize = SCREEN_HEIGHT - PLATFORM_PART.len();

// Tiles for clouds.
const CLOUD_PART: [u128; 2] = [
    0b00000000000000000000000000000011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
];

#[derive(Default)]
pub struct Game;

impl Game {
    pub fn next(&self, tick: &u64) -> [u128; SCREEN_HEIGHT] {
        let mut buffer = [0 as u128; SCREEN_HEIGHT];

        // Clouds
        let offset = tick / 3 % 128;
        for y in 0..CLOUD_PART.len() {
            buffer[y + 2] = (CLOUD_PART[y] << offset) | (CLOUD_PART[y] >> (127 - offset));
            buffer[y + 13] = ((CLOUD_PART[y] >> 64) << offset) | ((CLOUD_PART[y] >> 64) >> (127 - offset));
        }

        // Platform
        let offset = tick % 128;
        for y in 0..PLATFORM_PART.len() {
            buffer[GROUND_LEVEL + y] = (PLATFORM_PART[y] << offset) | (PLATFORM_PART[y] >> (127 - offset));
        }

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
