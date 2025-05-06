use std::time::Duration;

pub struct DdsData {
    pub signal_data: Vec<(f64, f64)>,
}

impl DdsData {
    pub fn new() -> Self {
        Self {
            signal_data: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, freq: f64, attenu: f64) {
        self.signal_data.push((freq, 10. / (attenu + 1.)));
    }

    pub fn remove_signal(&mut self, freq: f64) {
        for i in 0..self.signal_data.len() {
            if self.signal_data[i].0 == freq {
                self.signal_data.remove(i);
                return;
            }
        }
    }
}

pub const MIN_FREQ: f64 = 4.08796875;
pub const MAX_FREQ: f64 = 6272.0;

const MIN_DELAY: Duration = Duration::from_millis(1);
const MAX_DELAY: Duration = Duration::from_millis(100);

pub fn convert_freq_to_tick_delay(freq: f64) -> Duration {
    if freq >= MAX_FREQ {
        return MAX_DELAY;
    } else if freq <= MIN_FREQ {
        return MIN_DELAY;
    } else {
        return MAX_DELAY
            - Duration::mul_f64(
                MAX_DELAY - MIN_DELAY,
                (freq - MIN_FREQ) / (MAX_FREQ - MIN_FREQ),
            );
    }
}
