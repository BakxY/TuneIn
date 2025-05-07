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
