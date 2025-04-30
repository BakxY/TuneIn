use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, List, ListDirection},
};
use serialport::{self, SerialPort};
use std::{io::Result, rc::Rc};
fn generate_layout(frame: &mut Frame) -> Rc<[Rect]> {
    let config_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());

    config_layout
}

fn render(frame: &mut Frame) {
    let config_layout = generate_layout(frame);

    frame.render_widget(
        Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default())
            .title("Port"),
        config_layout[0],
    );
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default())
            .title("Config"),
        config_layout[1],
    );
}
//udev rule
//KERNEL=="ttyUSB*", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE:="0666"
fn serial_connect() -> io::Result<Box<dyn SerialPort>> {
    let ports = serialport::available_ports().expect("no divices found");
    frame.render_widget(popup, popup_area);
    for _ in 0..42 {
        let n = ports.len();
        println!("{}", n);
        for (i, p) in ports.iter().enumerate() {
            println!("{}: {}", i + 1, p.port_name);
        }
        println!("Please select which Port to use:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");
        let input: usize = input.trim().parse().unwrap_or(0);
        println!("{}", input);
        if input > 0 && input <= n {
            return Ok(serialport::new(ports[input - 1].port_name.clone(), 31_250)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port"));
        } else {
            println!("Input is invalid please use the number of a port")
        }
    }
    panic!("WTF are you doing just use a valid number");
}

fn print_events(mut port: Box<dyn SerialPort>) -> io::Result<()> {
    loop {
        let mut midi_frame: [u8; 3] = [0x00, 0x00, 0x00];
        // Blocking read
        let event = read()?;

        if let Event::Key(key_event) = event {
            if let KeyEventKind::Press = key_event.kind {
                midi_frame[0] = 0x90;
            } else if let KeyEventKind::Release = key_event.kind {
                midi_frame[0] = 0x80;
            }
            if let KeyCode::Char(c) = key_event.code {
                match c {
                    'a' => {
                        midi_frame[1] = 0x0c * 1;
                        midi_frame[2] = 0x60
                    }
                    's' => {
                        midi_frame[1] = 0x0c * 2;
                        midi_frame[2] = 0x60
                    }
                    'd' => {
                        midi_frame[1] = 0x0c * 3;
                        midi_frame[2] = 0x60
                    }
                    'f' => {
                        midi_frame[1] = 0x0c * 4;
                        midi_frame[2] = 0x60
                    }
                    'g' => {
                        midi_frame[1] = 0x0c * 5;
                        midi_frame[2] = 0x60
                    }
                    'h' => {
                        midi_frame[1] = 0x0c * 6;
                        midi_frame[2] = 0x60
                    }
                    'j' => {
                        midi_frame[1] = 0x0c * 7;
                        midi_frame[2] = 0x60
                    }
                    'k' => {
                        midi_frame[1] = 0x0c * 8;
                        midi_frame[2] = 0x60
                    }
                    'l' => {
                        midi_frame[1] = 0x0c * 9;
                        midi_frame[2] = 0x60
                    }
                    'ö' => {
                        midi_frame[1] = 0x0c * 10;
                        midi_frame[2] = 0x60
                    }
                    _ => {
                        midi_frame[1] = 0x0c * 7;
                        midi_frame[2] = 0x60
                    }
                }
                port.write(&midi_frame).expect("write failed");
                println!(
                    "Status: {:#x} Note: {:#x} Value: {:#x}\r",
                    midi_frame[0], midi_frame[1], midi_frame[2]
                );
            }
        }
        println!("Event: {:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }
        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }
    Ok(())
}
