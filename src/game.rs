use core::cmp::min;

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 64;

const PLATFORM_WIDTH: usize = 64;
const PLATFORM_HEIGHT: usize = 3;
const PLATFORM_START: usize = SCREEN_HEIGHT - PLATFORM_HEIGHT;

const DINO_WIDTH: usize = 8;
const DINO_HEIGHT: usize = 8;

// (6, 10) in
// https://www.reddit.com/r/PixelArt/comments/kzqite/oc_cute_8x8_pixel_art_with_max_3_colours_per/#lightbox
const DINO_TILE: u64 = 0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010100;

// Having a repeating sequence is simpler than random generation at runtime.
const GRAVEL_WIDTH: usize = 64;
const GRAVEL_TILE: u64 = 0b00110001_01000010_10011001_01011001_01100111_10000000_00000100_11101110;

#[derive(Default)]
pub struct Game;

impl Game {
    pub fn next(&self, tick: &u64) -> [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        let mut buffer = [false; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Platform
        // Ground
        for x in 0..128 {
            buffer[PLATFORM_START * SCREEN_WIDTH + x] = true;
        }

        // Gravel
        let offset = ((tick % 128) % 64) as usize;
        for x in offset..64 {
            let pixel = ((GRAVEL_TILE >> (GRAVEL_WIDTH - 1 - x)) & 1) == 1;
            buffer[(PLATFORM_START + 2) * SCREEN_WIDTH + (x - offset)] = pixel;
        }

        let mut spanned = PLATFORM_WIDTH - offset;
        while spanned < SCREEN_WIDTH {
            let cover = min(SCREEN_WIDTH - spanned, PLATFORM_WIDTH);
            for x in 0..cover {
                let pixel = ((GRAVEL_TILE >> (GRAVEL_WIDTH - 1 - x)) & 1) == 1;
                buffer[(PLATFORM_START + 2) * SCREEN_WIDTH + spanned + x] = pixel;
            }
            spanned += cover;
        }

        // Character
        for y in 0..DINO_HEIGHT {
            for x in 0..DINO_WIDTH {
                let pixel = ((DINO_TILE >> ((DINO_HEIGHT * DINO_WIDTH - 1) - (y * DINO_WIDTH + x))) & 1) == 1;
                buffer[(PLATFORM_START - DINO_HEIGHT + y) * SCREEN_WIDTH + x] = pixel;
            }
        }

        buffer
    }
}
