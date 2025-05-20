pub fn freq_to_note_id(freq: f64) -> u8 {
    return (12.0 * (freq / 440.0).log2() + 69.0).round() as u8;
}