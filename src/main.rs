use crossterm::event::{self, Event, KeyCode};
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
mod serial;
mod render_utils;

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

        self.com_config.scan_serialports();

        loop {
            let _ = terminal.draw(|frame| self.draw(frame));

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        return Ok(());
                    }

                    match self.state {
                        AppState::Running => match key.code {
                            KeyCode::Char('q') => break Ok(()),
                            KeyCode::Char('p') => {
                                self.state = AppState::ComConfig;
                                self.com_config.scan_serialports();
                            }
                            KeyCode::Char('w') => {
                                self.dds_config.toggle_signal(6000., 1.);
                            }
                            KeyCode::Char('e') => {
                                self.dds_config.toggle_signal(4000., 3.);
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
