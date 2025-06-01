use core::cmp::min;

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 64;

// (6, 10) in
// https://www.reddit.com/r/PixelArt/comments/kzqite/oc_cute_8x8_pixel_art_with_max_3_colours_per/#lightbox
// This is already left shifted because we know that this tile will appear on the left of the screen.
const DINO_STILL_TILE: u128 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010100 << 64;
const DINO_WALK_LEFT: u128 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010000 << 64;
const DINO_WALK_RIGHT: u128 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00000100 << 64;
const DINO_WIDTH: usize = 8;
const DINO_HEIGHT: usize = 8;

// Having a repeating sequence is simpler than random generation at runtime.
const PLATFORM_WIDTH: usize = 64;
const PLATFORM_HEIGHT: usize = 3;
const PLATFORM_START: usize = SCREEN_HEIGHT - PLATFORM_HEIGHT;
const GRAVEL_TILE_64: u128 = 0b00110001_01000010_10011001_01011001_01100111_10000000_00000100_11101110;
const GRAVEL_TILE: u128 = (GRAVEL_TILE_64 << 64) | GRAVEL_TILE_64 as u128;

// A row of clouds.
const CLOUD_TILES: [u128; 2] = [
    0b00000000000000000000000000000011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
];

#[derive(Default)]
pub struct Game;

impl Game {
    pub fn next(&self, tick: &u64) -> [u128; SCREEN_HEIGHT] {
        let mut buffer = [0 as u128; SCREEN_HEIGHT];

        // Clouds that move at a slower rate.
        let offset = tick / 3 % 128;
        for y in 0..CLOUD_TILES.len() {
            buffer[y + 2] = (CLOUD_TILES[y] << offset) | (CLOUD_TILES[y] >> (127 - offset));
            // The second row of clouds are right aligned.
            buffer[y + 13] = ((CLOUD_TILES[y] >> 64) << offset) | ((CLOUD_TILES[y] >> 64) >> (127 - offset));
        }

        // Platform
        // Ground
        buffer[PLATFORM_START] = u128::MAX;

        // Gravel
        let offset = ((tick % 128) % 64) as usize;
        buffer[PLATFORM_START + 2] =
            // Tile with offset.
            (GRAVEL_TILE << offset)
            // Pad the end.
            | (GRAVEL_TILE_64 >> (64 - offset));

        // Character
        for y in 0..DINO_HEIGHT {
            let abs_y = PLATFORM_START - DINO_HEIGHT + y;

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
