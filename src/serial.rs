use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, List, ListState, Row, Table},
};
use serialport::{self, SerialPort};
use std::time::Duration;

use crate::AppState;
use crate::input::Input;

#[derive(Debug, PartialEq, Eq)]
enum ConfigState {
    PortSelection,
    BaudSelection,
}

pub struct ComConfig {
    config_state: ConfigState,
    list_state: ListState,
    com_ports: Vec<serialport::SerialPortInfo>,
    port_index: usize,
    active_com_port: Option<Box<dyn SerialPort>>,
    baud: u32,
    input: Input,
}

impl ComConfig {
    //Constructor
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

    pub fn send_midi(&mut self, status: u8, note: u8, vel: u8) {
        // Todo no Com
        self.active_com_port
            .as_mut()
            .unwrap()
            .write(&[status, note, vel])
            .expect("Send failed");
    }

    pub fn key_event(&mut self, key: KeyEvent) -> AppState {
        let mut app_state: AppState = AppState::ComConfig;
        match self.config_state {
            ConfigState::PortSelection => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => app_state = AppState::Running,
                KeyCode::Tab => self.toggle_state(),
                KeyCode::Enter => match self.list_state.selected() {
                    Some(i) => {
                        self.port_index = i;
                        self.config_state = ConfigState::BaudSelection;
                    }
                    None => {}
                },
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('r') => self.scan_serialports(),
                _ => {}
            },
            ConfigState::BaudSelection => match key.code {
                KeyCode::Tab => self.toggle_state(),
                KeyCode::Enter => {
                    // Todo Error handling
                    match self.input.submit_message().parse() {
                        Ok(b) => {
                            self.baud = b;
                            self.active_com_port = Some(
                                serialport::new(
                                    self.com_ports[self.port_index].port_name.clone(),
                                    self.baud,
                                )
                                .timeout(Duration::from_millis(10))
                                .open()
                                .expect("Failed to open port"),
                            );
                            self.config_state = ConfigState::BaudSelection;
                            app_state = AppState::Running;
                        }
                        Err(_) => self.input.display_error(String::from("Not a valid value")),
                    }
                }
                _ => {
                    if self.input.key_event(key) {
                        app_state = AppState::Running;
                    }
                }
            },
        };
        return app_state;
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }
    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }
    fn toggle_state(&mut self) {
        match self.config_state {
            ConfigState::PortSelection => self.config_state = ConfigState::BaudSelection,
            ConfigState::BaudSelection => self.config_state = ConfigState::PortSelection,
        }
    }

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

        let area = popup_area(frame.area(), 60, 40);

        frame.render_widget(Clear, area); //this clears out the background
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(3)])
            .split(area);
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
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn get_rows_nc() -> [Row<'static>; 2] {
    [
        Row::new(vec![String::from("Name"), "Not connected".to_string()]).red(),
        Row::new(vec!["Baud".to_string(), "Not connected".to_string()]).red(),
    ]
}
