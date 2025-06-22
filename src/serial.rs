use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, List, ListState, Paragraph, Row, Table},
};
use serialport::{self, SerialPort};
use std::{rc::Rc, time::Duration};

use crate::AppState;
use crate::input::Input;
use crate::popup_utils::popup_area;

// Different states
#[derive(Debug, PartialEq, Eq)]
enum ConfigState {
    PortSelection, //Selecting port
    BaudSelection, //Selecting baud
}
// Main com config struct
pub struct ComConfig {
    config_state: ConfigState,                    //Current state
    list_state: ListState,                        //State of the list
    com_ports: Vec<serialport::SerialPortInfo>,   //List of all available com ports
    port_index: usize,                            //Index of selected port
    active_com_port: Option<Box<dyn SerialPort>>, //Connected com port
    baud: u32,                                    //Desired baud
    input: Input,                                 //Input for text input
}

impl ComConfig {
    //Constructor with default values
    pub fn new() -> Self {
        Self {
            config_state: ConfigState::PortSelection,
            list_state: ListState::default(),
            com_ports: Vec::new(),
            port_index: 0,
            active_com_port: None,
            baud: 0,
            input: Input::new(),
        }
    }

    //Looks for Com ports
    //udev rule
    //KERNEL=="ttyUSB*", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE:="0666"
    pub fn scan_serialports(&mut self) {
        self.com_ports = serialport::available_ports().expect("Error reading Com ports");
    }
    // Send a midi message
    pub fn send_midi(&mut self, status: u8, note: u8, vel: u8) {
        self.active_com_port
            .as_mut()
            .unwrap()
            .write(&[status, note, vel])
            .expect("Send failed");
    }
    // Event handling
    pub fn key_event(&mut self, key: KeyEvent) -> AppState {
        let mut app_state: AppState = AppState::ComConfig;
        match self.config_state {
            ConfigState::PortSelection => match key.code {
                // Quit
                KeyCode::Char('q') | KeyCode::Esc => app_state = AppState::Running,
                // Toggle stat
                KeyCode::Tab => self.toggle_state(),
                // Select entry
                KeyCode::Enter => match self.list_state.selected() {
                    Some(i) => {
                        self.port_index = i;
                        self.config_state = ConfigState::BaudSelection;
                    }
                    None => {}
                },
                // Move down
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                // Move up
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                // Scan for serial ports
                KeyCode::Char('r') => self.scan_serialports(),
                _ => {}
            },
            ConfigState::BaudSelection => match key.code {
                // Toggle stat
                KeyCode::Tab => self.toggle_state(),
                // Submit settings
                KeyCode::Enter => match self.input.submit_message().parse() {
                    // Check if input is valid
                    Ok(b) => {
                        self.baud = b;
                        // Connect to the port
                        self.active_com_port = Some(
                            serialport::new(
                                self.com_ports[self.port_index].port_name.clone(),
                                self.baud,
                            )
                            .timeout(Duration::from_millis(10))
                            .open()
                            .expect("Failed to open port"),
                        );
                        // Change state
                        self.config_state = ConfigState::BaudSelection;
                        app_state = AppState::Running;
                    }
                    Err(_) => self.input.display_error(String::from("Not a valid value")),
                },
                _ => {
                    if self.input.key_event(key) {
                        app_state = AppState::Running;
                    }
                }
            },
        };
        return app_state;
    }
    // Select next entry
    fn select_next(&mut self) {
        self.list_state.select_next();
    }
    // Select prev entry
    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }
    // Toggle state
    fn toggle_state(&mut self) {
        match self.config_state {
            ConfigState::PortSelection => self.config_state = ConfigState::BaudSelection,
            ConfigState::BaudSelection => self.config_state = ConfigState::PortSelection,
        }
    }
    // Get table for rendering
    pub fn get_table(&self) -> Table<'_> {
        // Create data rows
        let rows = if let Some(p) = &self.active_com_port {
            if let Ok(r) = p.baud_rate() {
                [
                    Row::new(vec![String::from("Name"), p.name().unwrap().clone()]).green(),
                    Row::new(vec!["Baud".to_string(), r.clone().to_string()]).green(),
                ]
            } else {
                get_rows_nc()
            }
        } else {
            get_rows_nc()
        };

        // Define how wide cells of table are
        let widths = [Constraint::Percentage(30), Constraint::Fill(1)];

        // Create table and the block surrounding it
        Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::default())
            .block(
                Block::new()
                    .border_type(BorderType::Thick)
                    .borders(Borders::ALL)
                    .title("Serial"),
            )
    }

    //Render a popup form Com settings
    pub fn show_com_popup(&mut self, frame: &mut Frame) {
        let list = List::new(
            self.com_ports
                .iter()
                .map(|port| Text::from(port.port_name.clone()))
                .collect::<Vec<Text>>(),
        )
        .block(
            Block::bordered()
                .title("Com Ports")
                .border_type(BorderType::Thick),
        )
        .style(Style::default())
        .highlight_style(Style::new().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::WhenSelected)
        .repeat_highlight_symbol(false);
        // Area of the popup
        let area = popup_area(frame.area(), 60, 40);
        // Clear area
        frame.render_widget(Clear, area); //this clears out the background
        // Layout
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(3)])
            .split(area);
        // Highlighting
        match self.config_state {
            ConfigState::PortSelection => {
                frame.render_stateful_widget(
                    &list.yellow(),
                    vertical_layout[0],
                    &mut self.list_state,
                );
                frame.render_widget(
                    self.input
                        .get_input(String::from("Baud"))
                        .style(Style::default()),
                    vertical_layout[1],
                );
            }
            ConfigState::BaudSelection => {
                frame.render_stateful_widget(&list, vertical_layout[0], &mut self.list_state);
                frame.render_widget(
                    self.input.get_input(String::from("Baud")),
                    vertical_layout[1],
                );
                // Turn cursor on
                frame.set_cursor_position(Position::new(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    area.x + self.input.get_index() + 1,
                    // Move one line down, from the border to the input line
                    area.y + area.height - 2,
                ));
            }
        }
    }
    // Render shortcuts
    pub fn render_shortcuts(&self, frame: &mut Frame, layout: Rc<[Rect]>) {
        let shortcuts = if self.config_state == ConfigState::PortSelection {
            "Quit Config: q | \
                    Switch to Baud entry: tab | \
                    Submit: Enter | \
                    Next Entry: Down/j | \
                    Prev Entry: Up/k | \
                    Rescan serialports: r"
        } else {
            self.input.get_shortcuts()
        };
        let paragraph = Paragraph::new(shortcuts).style(Style::new().blue());
        frame.render_widget(paragraph, layout[1]);
    }
}

fn get_rows_nc() -> [Row<'static>; 2] {
    [
        Row::new(vec![String::from("Name"), "Not connected".to_string()]).red(),
        Row::new(vec!["Baud".to_string(), "Not connected".to_string()]).red(),
    ]
}
