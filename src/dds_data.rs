#[derive(Copy, Clone)]
pub struct DdsData {
    freq: f64,
    attenu: u32,
    on: bool,
    pub signal_data: [(f64, f64); 19],
}

impl DdsData {
    pub fn new() -> Self {
        Self {
            freq: 0.,
            attenu: 0,
            on: false,
            signal_data: get_off_signal()
        }
    }

    pub fn enable_signal(&mut self) {
        self.on = true;
        self.signal_data = get_sin_signal();
    }

    pub fn disable_signal(&mut self) {
        self.on = false;
        self.signal_data = get_off_signal();
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