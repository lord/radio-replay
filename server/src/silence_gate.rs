const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;

pub struct SilenceGate {
    sound_count: usize,
}

impl SilenceGate {
    pub fn new() -> Self {
        Self { sound_count: 0 }
    }

    pub fn add_sound(&mut self, data: &[i32]) {
        self.sound_count += data.len();
    }

    pub fn is_open(&self) -> bool {
        (self.sound_count % 420000) < 300000
    }
}
