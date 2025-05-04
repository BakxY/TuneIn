use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Row, Table},
};
use serial::ComConfig;
use std::{
    io::Result,
    rc::Rc,
    time::{Duration, Instant},
};

mod input;
mod serial;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = TuneIn::new().run(terminal);
    ratatui::restore();
    result
}

struct TuneIn {
    dds_signal_data: [[(f64, f64); 19]; 10],
}

struct DdsSignal {}

impl DdsSignal {
    fn get_off_signal() -> [(f64, f64); 19] {
        return [
            (0., 0.),
            (5.2632, 0.),
            (10.5263, 0.),
            (15.7895, 0.),
            (21.0526, 0.),
            (26.3158, 0.),
            (31.5789, 0.),
            (36.8421, 0.),
            (42.1053, 0.),
            (47.3684, 0.),
            (52.6316, 0.),
            (57.8947, 0.),
            (63.1579, 0.),
            (68.4211, 0.),
            (73.6842, 0.),
            (78.9474, 0.),
            (84.2105, 0.),
            (89.4737, 0.),
            (100., 0.),
        ];
    }

    fn get_sin_signal() -> [(f64, f64); 19] {
        return [
            (0., 0.),
            (5.2632, 1.6235),
            (10.5263, 3.0711),
            (15.7895, 4.1858),
            (21.0526, 4.8470),
            (26.3158, 4.9829),
            (31.5789, 4.5789),
            (36.8421, 3.6786),
            (42.1053, 2.3797),
            (47.3684, 0.8230),
            (52.6316, -0.8230),
            (57.8947, -2.3797),
            (63.1579, -3.6786),
            (68.4211, -4.5789),
            (73.6842, -4.9829),
            (78.9474, -4.8470),
            (84.2105, -4.1858),
            (89.4737, -3.0711),
            (94.7368, -1.6235),
        ];
    }
}

impl TuneIn {
    fn new() -> Self {
        let mut default_data: [[(f64, f64); 19]; 10] = [[(0_f64, 0_f64); 19]; 10];

        for i in 0..10 {
            default_data[i] = DdsSignal::get_off_signal();
        }

        default_data[0] = DdsSignal::get_sin_signal();

        Self {
            dds_signal_data: default_data,
        }
    }

    fn cycle_sin_data(&mut self, id: usize) {
        let cycled_item = self.dds_signal_data[id][0].1;

        for i in 0..self.dds_signal_data[id].len() - 1 {
            self.dds_signal_data[id][i].1 = self.dds_signal_data[id][i + 1].1;
        }

        self.dds_signal_data[id][self.dds_signal_data[id].len() - 1].1 = cycled_item;
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(10);
        let mut last_tick = Instant::now();
        let mut com_config = ComConfig::new();

        loop {
            let _ = terminal.draw(|frame| self.draw(frame, &mut com_config));
            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break Ok(());
                    }
                    else {
                        com_config.key_event(key);
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        self.cycle_sin_data(0);
    }

    fn generate_layout(frame: &mut Frame) -> (Rc<[Rect]>, Vec<Rect>) {
        let vertical_temp_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame.area());

        let general_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(vertical_temp_layout[0]);

        let midi_temp_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(vertical_temp_layout[1]);

        let mut midi_layout: Vec<Rect> = Vec::new();

        midi_layout.append(
            &mut Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ])
                .split(midi_temp_layout[0])
                .to_vec(),
        );

        midi_layout.append(
            &mut Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ])
                .split(midi_temp_layout[1])
                .to_vec(),
        );

        return (general_layout, midi_layout);
    }

    fn split_midi_layout(layout: Rect) -> Vec<Rect> {
        let split_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(2)
            .split(layout)
            .to_vec();

        return split_layout;
    }

    fn draw(&self, frame: &mut Frame, com_config: &mut ComConfig) {
        let (general_layout, midi_layout) = TuneIn::generate_layout(frame);

        frame.render_widget(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default())
                .title("Info"),
            general_layout[0],
        );
        frame.render_widget(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default())
                .title("Options"),
            general_layout[1],
        );
        frame.render_widget(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default())
                .title("Communication"),
            general_layout[2],
        );

        let rows = [
            Row::new(vec!["Note", "IDFK"]),
            Row::new(vec!["Freq", "69 kHz"]),
            Row::new(vec!["Atten", "2"]),
        ];
        // Columns widths are constrained in the same way as Layout...
        let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
        let table = Table::new(rows, widths)
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            // You can set the style of the entire Table.
            .style(Style::new().white())
            // As any other widget, a Table can be wrapped in a Block.
            .block(Block::new());

        for i in 0..midi_layout.len() {
            let root_block = Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default())
                .title("DDS ".to_string() + &(i + 1).to_string());

            frame.render_widget(root_block.clone(), midi_layout[i]);

            let split_layout = TuneIn::split_midi_layout(midi_layout[i]);

            let dataset = Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::new().fg(Color::Red))
                .graph_type(GraphType::Line)
                .data(&self.dds_signal_data[i]);

            let chart = Chart::new(vec![dataset.clone()])
                .block(Block::new().borders(Borders::ALL))
                .x_axis(Axis::default().bounds([0.0, 100.0]))
                .y_axis(Axis::default().bounds([-5.0, 5.0]));

            frame.render_widget(chart, split_layout[0]);
            frame.render_widget(table.clone(), split_layout[1]);
        }
        com_config.scan_serialports();
        com_config.show_com_popup(frame);
    }
}
