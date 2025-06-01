use crate::rng::LFSR;

const SCREEN_HEIGHT: usize = 64;
const GROUND_LEVEL: usize = SCREEN_HEIGHT - 3;

type Buffer = [u128; SCREEN_HEIGHT];

pub struct Game<'a> {
    rng: &'a mut LFSR,

    gravel: u128,
    clouds_offsets: [(u8, u8); 2],
}

impl<'a> Game<'a> {
    pub fn new(rng: &'a mut LFSR) -> Self {
        let gravel = rng.next_u32() as u128
            | ((rng.next_u32() as u128) << 32)
            | ((rng.next_u32() as u128) << 64)
            | ((rng.next_u32() as u128) << 96);

        let clouds_offsets = [
            ((64 + (rng.next_u32() % 64)) as u8, 2),
            ((rng.next_u32() % 64) as u8, 12),
        ];

        Game {
            rng,
            gravel,
            clouds_offsets,
        }
    }

    fn render_platform(&mut self, _tick: &u64, buffer: &mut Buffer) {
        self.gravel = (self.gravel << 1) | (self.rng.next_bit() as u128);
        buffer[GROUND_LEVEL] = u128::MAX;
        buffer[GROUND_LEVEL + 1] = 0;
        buffer[GROUND_LEVEL + 2] = self.gravel;
    }

    fn render_clouds(&mut self, tick: &u64, buffer: &mut Buffer) {
        let offset = tick / 3 % 128;
        for (x_offset, y_offset) in self.clouds_offsets {
            let tile: u128 = 0b0110 << x_offset;
            buffer[y_offset as usize] = (tile << offset) | (tile >> (127 - offset));

            let tile: u128 = 0b1111 << x_offset;
            buffer[y_offset as usize + 1] = (tile << offset) | (tile >> (127 - offset));
        }
    }

    fn render_dino(&mut self, tick: &u64, buffer: &mut Buffer) {
        // (6, 10) in
        // https://www.reddit.com/r/PixelArt/comments/kzqite/oc_cute_8x8_pixel_art_with_max_3_colours_per/#lightbox
        // This is already left shifted because we know that this tile will appear on the left of the screen.
        const STILL_TILE: u128 =
            0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010100 << 64;

        const WALK_RIGHT: u128 =
            0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00000100 << 64;

        const WALK_LEFT: u128 =
            0b00011110_00010101_00011111_00011100_01011110_01011100_00111100_00010000 << 64;

        for y in 0..8 {
            // Animate walk with an either freq.
            let tile = if tick / 3 % 2 == 1 {
                WALK_LEFT
            } else {
                WALK_RIGHT
            };

            let abs_y = GROUND_LEVEL - 8 + y;
            buffer[abs_y] = buffer[abs_y]
                // Select the byte from the tile.
                | ((tile << 8 * y) & (0xff << 120)) >> 2;
        }
    }

    pub fn next(&mut self, tick: &u64) -> Buffer {
        let mut buffer: Buffer = [0 as u128; 64];

        self.render_platform(tick, &mut buffer);
        self.render_clouds(tick, &mut buffer);
        self.render_dino(tick, &mut buffer);

        buffer
    }
}
