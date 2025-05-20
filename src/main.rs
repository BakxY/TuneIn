use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use dds_data::DdsData;
use ratatui::{
    DefaultTerminal, Frame,
};
use serial::ComConfig;
use std::{
    io::Result,
    time::{Duration, Instant},
};

mod dds_data;
mod input;
mod layout_utils;
mod render_utils;
mod serial;
mod midi_utils;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = TuneIn::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(PartialEq, Eq)]
pub enum AppState {
    Running = 0,
    ComConfig = 1,
}

struct TuneIn {
    state: AppState,
    dds_config: dds_data::DdsData,
    com_config: serial::ComConfig,
    current_attenu: f64,
    current_octave: i32,
}

impl TuneIn {
    fn new() -> Self {
        Self {
            state: AppState::Running,
            dds_config: DdsData::new(),
            com_config: ComConfig::new(),
            current_attenu: 255.,
            current_octave: 0,
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(1);
        let mut last_tick = Instant::now();

        self.com_config.scan_serialports();

        loop {
            let _ = terminal.draw(|frame| self.draw(frame));

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return Ok(());
                    }

                    match self.state {
                        AppState::Running => match key.code {
                            KeyCode::Char('q') => break Ok(()),
                            KeyCode::Char('p') => {
                                self.state = AppState::ComConfig;
                                self.com_config.scan_serialports();
                            }
                            KeyCode::Char('s') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    261.63 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('d') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    293.66 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('f') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    329.63 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('g') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    349.23 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('h') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    392.00 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('j') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    440.00 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('k') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    493.88 * f64::powi(2., self.current_octave),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('l') => {
                                self.dds_config.toggle_signal(
                                    &mut self.com_config,
                                    261.63 * f64::powi(2., self.current_octave + 1),
                                    self.current_attenu,
                                );
                            }
                            KeyCode::Char('v') => {
                                if self.current_attenu > 0. {
                                    self.current_attenu -= 5.;
                                }
                            }
                            KeyCode::Char('V') => {
                                if self.current_attenu < 255. {
                                    self.current_attenu += 5.;
                                }
                            }
                            KeyCode::Char('n') => {
                                if self.current_octave > -6 {
                                    self.current_octave -= 1;
                                }
                            }
                            KeyCode::Char('N') => {
                                if self.current_octave < 4 {
                                    self.current_octave += 1;
                                }
                            }
                            KeyCode::Char('c') => {
                                for tone in self.dds_config.signal_data.clone() {
                                    self.dds_config.remove_signal(&mut self.com_config, tone.0);
                                }
                            }
                            _ => {}
                        },

                        AppState::ComConfig => self.state = self.com_config.key_event(key),
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {}

    fn draw(&mut self, frame: &mut Frame) {
        let (general_layout, fft_layout, channel_layout) =
            layout_utils::generate_main_layout(frame);
        let serial_table = self.com_config.get_table();

        render_utils::render_general(frame, general_layout, serial_table, self.current_attenu, self.current_octave);
        render_utils::render_dds(frame, fft_layout, &self.dds_config.signal_data);
        render_utils::render_channels(frame, channel_layout, &self.dds_config.signal_data);

        if self.state == AppState::ComConfig {
            self.com_config.show_com_popup(frame);
        }
    }
}
