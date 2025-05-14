use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use dds_data::DdsData;
use ratatui::{
    DefaultTerminal, Frame,
    layout::Constraint,
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Padding, Row, Table},
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
}

impl TuneIn {
    fn new() -> Self {
        Self {
            state: AppState::Running,
            dds_config: DdsData::new(),
            com_config: ComConfig::new(),
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(1);
        let mut last_tick = Instant::now();

        let mut current_attenu = 0.;
        let mut current_octave = 0;

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
                                    261.63 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('d') => {
                                self.dds_config.toggle_signal(
                                    293.66 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('f') => {
                                self.dds_config.toggle_signal(
                                    329.63 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('g') => {
                                self.dds_config.toggle_signal(
                                    349.23 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('h') => {
                                self.dds_config.toggle_signal(
                                    392.00 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('j') => {
                                self.dds_config.toggle_signal(
                                    440.00 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('k') => {
                                self.dds_config.toggle_signal(
                                    493.88 * f64::powi(2., current_octave),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('l') => {
                                self.dds_config.toggle_signal(
                                    261.63 * f64::powi(2., current_octave + 1),
                                    current_attenu,
                                );
                            }
                            KeyCode::Char('v') => {
                                if current_attenu < 10. {
                                    current_attenu -= 1.;
                                }
                            }
                            KeyCode::Char('V') => {
                                if current_attenu > 0. {
                                    current_attenu += 1.;
                                }
                            }
                            KeyCode::Char('n') => {
                                if current_octave < 4 {
                                    current_octave -= 1;
                                }
                            }
                            KeyCode::Char('N') => {
                                if current_octave > -6 {
                                    current_octave += 1;
                                }
                            }
                            KeyCode::Char('c') => {
                                for tone in self.dds_config.signal_data.clone() {
                                    self.dds_config.remove_signal(tone.0);
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

        render_utils::render_general(frame, general_layout);
        render_utils::render_dds(frame, fft_layout, &self.dds_config.signal_data);
        render_utils::render_channels(frame, channel_layout, &self.dds_config.signal_data);

        if self.state == AppState::ComConfig {
            self.com_config.show_com_popup(frame);
        }
    }
}
