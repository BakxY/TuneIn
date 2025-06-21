use crate::{midi_utils, serial::ComConfig};
use rand::Rng;
// DDS main struct
pub struct DdsData {
    pub signal_data: Vec<(f64, f64)>, //Current DDS data
    rand: bool,
}

impl DdsData {
    // Create new struct
    pub fn new() -> Self {
        Self {
            signal_data: Vec::new(),
            rand: false,
        }
    }
    // Add a signal to the dds vec and send the midi message for it
    pub fn add_signal(&mut self, com_config: &mut ComConfig, freq: f64, mut strength: f64) {
        if self.rand {
            strength = rand::rng().random();
        }
        // Check for duplicats and if ther is space left
        if self.signal_data.len() < 10 && !self.signal_data.contains(&(freq, strength)) {
            // Send the midi Message to turn tone on
            com_config.send_midi(0x90, midi_utils::freq_to_note_id(freq), strength as u8);
            // Add to vec
            self.signal_data.push((freq, strength));
        }
    }
    // Remove a signal from the dds vec and turn the tone off
    pub fn remove_signal(&mut self, com_config: &mut ComConfig, freq: f64) {
        // Searche for signal
        for i in 0..self.signal_data.len() {
            if self.signal_data[i].0 == freq {
                // Turn tone off
                com_config.send_midi(0x80, midi_utils::freq_to_note_id(freq), 0x00);
                // Remove entry
                self.signal_data.remove(i);
                return;
            }
        }
    }
    // Toggle a signal
    pub fn toggle_signal(&mut self, com_config: &mut ComConfig, freq: f64, strength: f64) {
        if self.signal_data.contains(&(freq, strength)) {
            self.remove_signal(com_config, freq);
        } else {
            self.add_signal(com_config, freq, strength);
        }
    }
    pub fn toggle_rand(&mut self){
        self.rand = !self.rand;
    }
}
