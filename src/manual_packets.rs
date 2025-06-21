use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect}, style::{Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Clear, Paragraph}, Frame
};
use std::rc::Rc;

use crate::AppState;
use crate::input::Input;
use crate::popup_utils::popup_area;
use crate::ComConfig;

enum PacketConfigState {
    CommandInput,
    NoteInput,
    VelocityInput,
}
// Main com config struct
pub struct ManualPackets {
    config_state: PacketConfigState,
    command_input: Input,
    note_input: Input,
    velocity_input: Input,
}

impl ManualPackets {
    //Constructor with default values
    pub fn new() -> Self {
        Self {
            config_state: PacketConfigState::CommandInput,
            command_input: Input::new(),
            note_input: Input::new(),
            velocity_input: Input::new(),
        }
    }
    // Event handling
    pub fn key_event(&mut self, key: KeyEvent, com_used: &mut ComConfig) -> AppState {
        let mut app_state: AppState = AppState::Manual;
        match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Esc => app_state = AppState::Running,
            // Toggle state
            KeyCode::Tab => self.toggle_state(),
            // Clear input fields
            KeyCode::Char('c') => {
                self.command_input.clear_input();
                self.note_input.clear_input();
                self.velocity_input.clear_input();
            }
            KeyCode::Enter => {
                let mut command: u8 = 0;
                let mut note: u8 = 0;
                let mut velocity: u8 = 0;
                let mut err_flag: bool = false;
                
                match self.command_input.submit_message().parse() {
                    // Check if input is valid
                    Ok(c) if c > 15 && c < 241 => {
                        command = c;
                    }
                    Ok(_) | Err(_) => {
                        self.command_input.display_error(String::from("Not a valid value")); 
                        err_flag |= true;
                    },
                }

                match self.note_input.submit_message().parse() {
                    // Check if input is valid
                    Ok(n) if n < 127 => {
                        note = n;
                    }
                    Ok(_) | Err(_) => {
                        self.note_input.display_error(String::from("Not a valid value")); 
                        err_flag |= true;
                    },
                }

                match self.velocity_input.submit_message().parse() {
                    // Check if input is valid
                    Ok(v) if v < 127 => {
                        velocity = v;
                    }
                    Ok(_) | Err(_) => {
                        self.velocity_input.display_error(String::from("Not a valid value")); 
                        err_flag |= true;
                    },
                }

                if !err_flag {
                    com_used.send_midi((command & 0xF0) | 0x80, note, velocity);
                }
            },
            _ => {
                match self.config_state {
                    PacketConfigState::CommandInput => {
                        if self.command_input.key_event(key) {
                            app_state = AppState::Running;
                        }
                    }
                    PacketConfigState::NoteInput => {
                        if self.note_input.key_event(key) {
                            app_state = AppState::Running;
                        }
                    }
                    PacketConfigState::VelocityInput => {
                        if self.velocity_input.key_event(key) {
                            app_state = AppState::Running;
                        }
                    }
                };
            }
        }
        return app_state;
    }
    // Toggle state
    fn toggle_state(&mut self) {
        match self.config_state {
            PacketConfigState::CommandInput => self.config_state = PacketConfigState::NoteInput,
            PacketConfigState::NoteInput => self.config_state = PacketConfigState::VelocityInput,
            PacketConfigState::VelocityInput => self.config_state = PacketConfigState::CommandInput,
        }
    }
    //Render a popup form Com settings
    pub fn show_manual_popup(&mut self, frame: &mut Frame) {
        // Area of the popup
        let area = popup_area(frame.area(), 60, 40);
        // Clear area
        frame.render_widget(Clear, area); //this clears out the background
        // Layout
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        let centered_title = format!("{:^width$}", "You are in manual mode.", width = vertical_layout[0].width as usize);

        frame.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from(Span::styled(centered_title, Style::default().add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from(" Build custom MIDI packets:"),
                Line::from(""),
                Line::from(" • Command: (Status byte, 16-241) defines the message type."),
                Line::from(" • Note: (Data byte, 0-127) specifies the pitch."),
                Line::from(" • Velocity: (Data byte, 0-127) indicates intensity."),
                Line::from(""),
                Line::from(" • Note: All packets will be sent on MIDI Channel 0 and command msb will always be set."),
            ]))
                .block(
                    Block::bordered()
                    .border_type(BorderType::Thick),
                ),
            vertical_layout[0],
        );

        frame.render_widget(
            self.command_input.get_input(String::from("Command"))
            .style(Style::default()),
            vertical_layout[1],
        );
        frame.render_widget(
            self.note_input
                .get_input(String::from("Note"))
                .style(Style::default()),
            vertical_layout[2],
        );
        frame.render_widget(
            self.velocity_input
                .get_input(String::from("Velocity"))
                .style(Style::default()),
            vertical_layout[3],
        );
        // Highlighting
        match self.config_state {
            PacketConfigState::CommandInput => {
                frame.render_widget(
            self.command_input.get_input(String::from("Command")),
                    vertical_layout[1],
                );
                frame.set_cursor_position(Position::new(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    vertical_layout[1].x + self.command_input.get_index() + 1,
                    // Move one line down, from the border to the input line
                    vertical_layout[1].y + vertical_layout[2].height - 2,
                ));
            }
            PacketConfigState::NoteInput => {
                frame.render_widget(
            self.note_input
                        .get_input(String::from("Note")),
                    vertical_layout[2],
                );
                frame.set_cursor_position(Position::new(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    vertical_layout[2].x + self.note_input.get_index() + 1,
                    // Move one line down, from the border to the input line
                    vertical_layout[2].y + vertical_layout[2].height - 2,
                ));
            }
            PacketConfigState::VelocityInput => {
                frame.render_widget(
            self.velocity_input
                        .get_input(String::from("Velocity")),
                    vertical_layout[3],
                );
                frame.set_cursor_position(Position::new(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    vertical_layout[3].x + self.velocity_input.get_index() + 1,
                    // Move one line down, from the border to the input line
                    vertical_layout[3].y + vertical_layout[2].height - 2,
                ));
            }
        }
    }
    // Render shortcuts
    pub fn render_shortcuts(&self, frame: &mut Frame, layout: Rc<[Rect]>) {
        let shortcuts = "Quit Manual: q | \
                                        Switch input field: tab | \
                                        Send: Enter | \
                                        Clear inputs: c";
        let paragraph = Paragraph::new(shortcuts).style(Style::new().blue());
        frame.render_widget(paragraph, layout[1]);
    }
}
