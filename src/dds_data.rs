use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
pub struct DdsData {
    pub freq: f64,
    pub last_cycle: Instant,
    pub attenu: u32,
    pub on: bool,
    pub signal_data: [(f64, f64); 19],
}

impl DdsData {
    pub fn new() -> Self {
        Self {
            freq: 0.,
            last_cycle: Instant::now(),
            attenu: 0,
            on: false,
            signal_data: get_off_signal(),
        }
    }

    pub fn enable_signal(&mut self) {
        self.on = true;
        self.signal_data = get_sin_signal();
    }

    pub fn disable_signal(&mut self) {
        self.on = false;
        self.signal_data = get_off_signal();

        self.attenu = 0;
        self.freq = 0.;
    }

    pub fn apply_attenuation(&mut self) {
        for i in 1..self.signal_data.len() {
            self.signal_data[i].1 = self.signal_data[i].1 / (self.attenu as f64 + 1.);
        }
    }
}

pub fn get_off_signal() -> [(f64, f64); 19] {
    return [
        (0., 0.),
        (5.2632, 0.),
        (10.5263, 0.),
        (15.7895, 0.),
        (21.0526, 0.),
        (26.3158, 0.),
        (31.5789, 0.),
        (36.8421, 0.),
        (42.1053, 0.),
        (47.3684, 0.),
        (52.6316, 0.),
        (57.8947, 0.),
        (63.1579, 0.),
        (68.4211, 0.),
        (73.6842, 0.),
        (78.9474, 0.),
        (84.2105, 0.),
        (89.4737, 0.),
        (100., 0.),
    ];
}

pub fn get_sin_signal() -> [(f64, f64); 19] {
    return [
        (0., 0.),
        (5.2632, 1.6235),
        (10.5263, 3.0711),
        (15.7895, 4.1858),
        (21.0526, 4.8470),
        (26.3158, 4.9829),
        (31.5789, 4.5789),
        (36.8421, 3.6786),
        (42.1053, 2.3797),
        (47.3684, 0.8230),
        (52.6316, -0.8230),
        (57.8947, -2.3797),
        (63.1579, -3.6786),
        (68.4211, -4.5789),
        (73.6842, -4.9829),
        (78.9474, -4.8470),
        (84.2105, -4.1858),
        (89.4737, -3.0711),
        (94.7368, -1.6235),
    ];
}

const MIN_FREQ: f64 = 4.08796875;
const MAX_FREQ: f64 = 6272.0;

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
