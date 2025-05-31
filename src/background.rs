use core::cmp::min;

const GRAVEL: [bool; 64] = [
    true,  false, false, false, true,  true,  false, false,
    false, true,  false, false, false, false, true,  false,
    true,  false, false, true,  true,  false, false, true,
    true,  false, false, true,  true,  false, true,  false,
    true,  true,  true,  false, false, true,  true,  false,
    false, false, false, false, false, false, false, true,
    false, false, true,  false, false, false, false, false,
    false, true,  true,  true,  false, true,  true,  true
];

// The background components on the game like the clouds, ground.
pub struct Background {
    global_tick: u64
}

impl Default for Background {
    fn default() -> Self {
        Self { global_tick: 0 }
    }
}

impl Background {
    pub fn next(&mut self) -> [bool; 128 * 3] {
        self.global_tick = self.global_tick.wrapping_add(1);

        // Ground and gravel.
        let mut platform = [false; 128 * 3];
        for x in 0..128 {
            platform[x] = true;
        }

        // Gravel
        let offset = ((self.global_tick % 128) % 64) as usize;
        for x in offset..64 {
            platform[256 + (x - offset)] = GRAVEL[x];
        }

        let mut spanned = 64 - offset;
        while spanned < 128 {
            let cover = min(128 - spanned, 64);
            for x in 0..cover {
                platform[256 + spanned + x] = GRAVEL[x];
            }
            spanned += cover;
        }

        platform
    }
}
