pub struct LcgRng {
    x: u32,
}

impl LcgRng {
    pub fn new(seed: u32) -> Self {
        Self { x: seed }
    }

    pub fn uniform(&mut self, low: f32, high: f32) -> f32 {
        self.x = 1664525 * self.x + 1013904223;
        low + self.x as f32 / u32::MAX as f32 * (high - low)
    }
}
