// Linear Feedback Shift Register (LFSR) based PRNG
// https://en.wikipedia.org/wiki/Linear-feedback_shift_register
pub struct LFSR {
    state: u32,
}

impl LFSR {
    /// Creates a new LFSR with the given non-zero seed
    pub const fn new(seed: u32) -> Self {
        LFSR { state: seed }
    }

    /// Generates the next bit and updates the internal state
    pub fn next_bit(&mut self) -> u32 {
        // Feedback polynomial: x^32 + x^22 + x^2 + x^1 + 1 (tap positions)
        let taps = (self.state >> 31) ^ (self.state >> 21) ^ (self.state >> 1) ^ (self.state >> 0);
        let new_bit = taps & 1;
        self.state = (self.state << 1) | new_bit;
        new_bit
    }

    /// Generates the next 32-bit pseudorandom number
    pub fn next_u32(&mut self) -> u32 {
        let mut result = 0;
        for _ in 0..32 {
            result = (result << 1) | self.next_bit();
        }
        result
    }
}
