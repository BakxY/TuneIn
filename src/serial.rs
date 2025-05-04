use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Clear, List},
};
use serialport::{self, SerialPort};

use crate::input::{self, Input};

#[derive(Debug)]
enum ConfigState {
    PortSelection,
    BaudSelection,
}

pub struct ComConfig {
    state: ConfigState,
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
            state: ConfigState::PortSelection,
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

    pub fn key_event(&mut self, key: KeyEvent) {
        match self.state {
            ConfigState::PortSelection => match key.code {
                KeyCode::Char('q') => {}
                KeyCode::Enter => {
                    // Todo Error handling
                    self.port_index = self.input.submit_message().parse().unwrap_or(0);
                    self.state = ConfigState::BaudSelection;
                }
                _ => self.input.key_event(key),
            },
            ConfigState::BaudSelection => match key.code {
                KeyCode::Char('q') => {}
                KeyCode::Enter => {
                    // Todo Error handling
                    self.baud = self.input.submit_message().parse().unwrap_or(0);
                    self.active_com_port = Some(
                        serialport::new(
                            self.com_ports[self.port_index].port_name.clone(),
                            self.baud,
                        )
                        .timeout(Duration::from_millis(10))
                        .open()
                        .expect("Failed to open port"),
                    );
                    self.state = ConfigState::BaudSelection;
                }
                _ => self.input.key_event(key),
            },
        };
    }
    //Render a popup form Com settings
    pub fn show_com_popup(&self, frame: &mut Frame) {
        let list = List::new(
            self.com_ports
                .iter()
                .enumerate()
                .map(|(n, port)| Text::from(n.to_string() + ": " + &port.port_name.clone()))
                .collect::<Vec<Text>>(),
        )
        .block(Block::bordered().title("Com Ports"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

        let area = popup_area(frame.area(), 60, 40);

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);
        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(list, vertical_layout[0]);
        frame.render_widget(self.input.get_input(), vertical_layout[1]);
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

// fn serial_connect() -> io::Result<Box<dyn SerialPort>> {
//     let ports = serialport::available_ports().expect("no divices found");
//     for _ in 0..42 {
//         let n = ports.len();
//         println!("{}", n);
//         for (i, p) in ports.iter().enumerate() {
//             println!("{}: {}", i + 1, p.port_name);
//         }
//         println!("Please select which Port to use:");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("failed to read");
//         let input: usize = input.trim().parse().unwrap_or(0);
//         println!("{}", input);
//         if input > 0 && input <= n {
//             return Ok(serialport::new(ports[input - 1].port_name.clone(), 31_250)
//                 .timeout(Duration::from_millis(10))
//                 .open()
//                 .expect("Failed to open port"));
//         } else {
//             println!("Input is invalid please use the number of a port")
//         }
//     }
//     panic!("WTF are you doing just use a valid number");
// }
//
// fn print_events(mut port: Box<dyn SerialPort>) -> io::Result<()> {
//     loop {
//         let mut midi_frame: [u8; 3] = [0x00, 0x00, 0x00];
//         // Blocking read
//         let event = read()?;
//
//         if let Event::Key(key_event) = event {
//             if let KeyEventKind::Press = key_event.kind {
//                 midi_frame[0] = 0x90;
//             } else if let KeyEventKind::Release = key_event.kind {
//                 midi_frame[0] = 0x80;
//             }
//             if let KeyCode::Char(c) = key_event.code {
//                 match c {
//                     'a' => {
//                         midi_frame[1] = 0x0c * 1;
//                         midi_frame[2] = 0x60
//                     }
//                     's' => {
//                         midi_frame[1] = 0x0c * 2;
//                         midi_frame[2] = 0x60
//                     }
//                     'd' => {
//                         midi_frame[1] = 0x0c * 3;
//                         midi_frame[2] = 0x60
//                     }
//                     'f' => {
//                         midi_frame[1] = 0x0c * 4;
//                         midi_frame[2] = 0x60
//                     }
//                     'g' => {
//                         midi_frame[1] = 0x0c * 5;
//                         midi_frame[2] = 0x60
//                     }
//                     'h' => {
//                         midi_frame[1] = 0x0c * 6;
//                         midi_frame[2] = 0x60
//                     }
//                     'j' => {
//                         midi_frame[1] = 0x0c * 7;
//                         midi_frame[2] = 0x60
//                     }
//                     'k' => {
//                         midi_frame[1] = 0x0c * 8;
//                         midi_frame[2] = 0x60
//                     }
//                     'l' => {
//                         midi_frame[1] = 0x0c * 9;
//                         midi_frame[2] = 0x60
//                     }
//                     'ö' => {
//                         midi_frame[1] = 0x0c * 10;
//                         midi_frame[2] = 0x60
//                     }
//                     _ => {
//                         midi_frame[1] = 0x0c * 7;
//                         midi_frame[2] = 0x60
//                     }
//                 }
//                 port.write(&midi_frame).expect("write failed");
//                 println!(
//                     "Status: {:#x} Note: {:#x} Value: {:#x}\r",
//                     midi_frame[0], midi_frame[1], midi_frame[2]
//                 );
//             }
//         }
//         println!("Event: {:?}\r", event);
//
//         if event == Event::Key(KeyCode::Char('c').into()) {
//             println!("Cursor position: {:?}\r", position());
//         }
//         if event == Event::Key(KeyCode::Esc.into()) {
//             break;
//         }
//     }
//     Ok(())
// }
