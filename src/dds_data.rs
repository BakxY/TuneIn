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
        if self.signal_data.len() < 10 && !self.signal_data.contains(&(freq, attenu)){
            self.signal_data.push((freq, attenu));
        }
    }

    pub fn remove_signal(&mut self, freq: f64) {
        for i in 0..self.signal_data.len() {
            if self.signal_data[i].0 == freq {
                self.signal_data.remove(i);
                return;
            }
        }
    }

    pub fn toggle_signal(&mut self, freq: f64, attenu: f64) {
        if self.signal_data.contains(&(freq, attenu)) {
            self.remove_signal(freq);
        }
        else {
            self.add_signal(freq, attenu);
        }
    }
}
