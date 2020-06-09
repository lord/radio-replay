const SILENCE_POWER_THRESHOLD: f64 = 1_000_000_000_000.0;
const SILENCE_SAMPLE_WINDOW: usize = 8000;

use std::collections::VecDeque;

pub struct SilenceGate {
    samples: VecDeque<f64>,
    total: f64,
}

impl SilenceGate {
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(SILENCE_SAMPLE_WINDOW),
            total: 0.0,
        }
    }

    pub fn add_sound(&mut self, data: &[i32]) {
        for datum in data {
            let v = *datum as f64 * *datum as f64;
            if self.samples.len() == SILENCE_SAMPLE_WINDOW {
                self.total -= self.samples.pop_front().unwrap();
            }
            self.total += v;
            self.samples.push_back(v);
        }
    }

    pub fn is_open(&self) -> bool {
        self.total / self.samples.len() as f64 > SILENCE_POWER_THRESHOLD
    }
}
