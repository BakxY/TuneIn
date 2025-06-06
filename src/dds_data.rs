use crate::{midi_utils, serial::ComConfig};

pub struct DdsData {
    pub signal_data: Vec<(f64, f64)>,
}

impl DdsData {
    pub fn new() -> Self {
        Self {
            signal_data: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, com_config: &mut ComConfig, freq: f64, strength: f64) {
        if self.signal_data.len() < 10 && !self.signal_data.contains(&(freq, strength)) {
            com_config.send_midi(
                    0x90,
                    midi_utils::freq_to_note_id(freq),
                    strength as u8,
                );

            self.signal_data.push((freq, strength));
        }
    }

    pub fn remove_signal(&mut self, com_config: &mut ComConfig, freq: f64) {
        for i in 0..self.signal_data.len() {
            if self.signal_data[i].0 == freq {
                com_config.send_midi(
                    0x80,
                    midi_utils::freq_to_note_id(freq),
                    0x00,
                );

                self.signal_data.remove(i);
                return;
            }
        }
    }

    pub fn toggle_signal(&mut self, com_config: &mut ComConfig, freq: f64, strength: f64) {
        if self.signal_data.contains(&(freq, strength)) {
            self.remove_signal(com_config, freq);
        } else {
            self.add_signal(com_config, freq, strength);
        }
    }
}
