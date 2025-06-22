use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use dds_data::DdsData;
use ratatui::{DefaultTerminal, Frame};
use serial::ComConfig;
use std::{
    io::Result,
    time::{Duration, Instant},
};

use crate::manual_packets::ManualPackets;

mod dds_data;
mod input;
mod layout_utils;
mod midi_utils;
mod render_utils;
mod serial;
mod manual_packets;
mod popup_utils;

//Entry Point
fn main() -> Result<()> {
    // Create a ratatui terminal
    let terminal = ratatui::init();
    // Start the main loop
    let result = TuneIn::new().run(terminal);
    // Stop execution
    ratatui::restore();
    result
}

// Main States
#[derive(PartialEq, Eq)]
pub enum AppState {
    // Running state where the user sends MIDI
    Running = 0,
    // Manual running state for sending packets with custom values
    Manual = 1,
    // Config state for selecting and configuring the com connection
    ComConfig = 2,
}

// Main App struct
struct TuneIn {
    state: AppState,                                //Main States
    dds_config: dds_data::DdsData,                  //DDS main struct
    com_config: serial::ComConfig,                  //Com main struct
    manual_config: manual_packets::ManualPackets,   //Manual mode main struct
    current_attenu: f64,                            //Current attenuatino for sending MIDI
    current_octave: i32,                            //Current octave for sending MIDI
}

impl TuneIn {
    // Create a new struct with default settings
    fn new() -> Self {
        Self {
            state: AppState::ComConfig,
            dds_config: DdsData::new(),
            com_config: ComConfig::new(),
            manual_config: ManualPackets::new(),
            current_attenu: 185.,
            current_octave: 1,
        }
    }
    // Run the Programm
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Refreshrate
        let tick_rate = Duration::from_millis(1);
        let mut last_tick = Instant::now();
        // Scan available serialports
        self.com_config.scan_serialports();
        // Goodloop
        loop {
            // Draw to the screen
            let _ = terminal.draw(|frame| self.draw(frame));
            // Poll every ms
            if event::poll(tick_rate)? {
                // Read key events
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    // Force quit
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return Ok(());
                    }
                    // Check who handle events currently
                    match self.state {
                        // Running state
                        AppState::Running => match key.code {
                            //Quit
                            KeyCode::Char('q') => break Ok(()),
                            //Change state to manual mode
                            KeyCode::Char('m') => {
                                self.state = AppState::Manual;
                            }
                            //Change state to ComConfig
                            KeyCode::Char('p') => {
                                self.state = AppState::ComConfig;
                                self.com_config.scan_serialports();
                            }
                            //Send Midi message
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
                            //Change attenuatino
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
                            //Change octave
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
                            KeyCode::Char('r') => {
                                self.dds_config.toggle_rand();
                            }
                            _ => {}
                        },
                        // Forward Keyevents to the manual subsystem
                        AppState::Manual => self.state = self.manual_config.key_event(key, &mut self.com_config),
                        // Forward Keyevents to the com subsystem
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
    // Draw the current frame
    fn draw(&mut self, frame: &mut Frame) {
        // Get the Layout
        let (base_layer, general_layout, fft_layout, channel_layout) =
            layout_utils::generate_main_layout(frame);
        let serial_table = self.com_config.get_table();
        // Render the main features
        render_utils::render_general(
            frame,
            general_layout,
            serial_table,
            self.current_attenu,
            self.current_octave,
        );
        render_utils::render_dds(frame, fft_layout, &self.dds_config.signal_data);
        render_utils::render_channels(frame, channel_layout, &self.dds_config.signal_data);

        // Show the com popup and shortcuts
        match self.state {
            AppState::Manual => {
                self.manual_config.show_manual_popup(frame);
                self.manual_config.render_shortcuts(frame, base_layer);
            },
            AppState::ComConfig => {
                self.com_config.show_com_popup(frame);
                self.com_config.render_shortcuts(frame, base_layer);
            },
            _ => {
                render_utils::render_shortcuts(frame, base_layer);
            }
        }
    }
}
