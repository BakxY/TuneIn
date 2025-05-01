use crossterm::event::{self, Event};
use dds_data::DdsData;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Row, Table},
};
use std::{
    io::Result,
    rc::Rc,
    time::{Duration, Instant},
};

pub mod dds_data;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = TuneIn::new().run(terminal);
    ratatui::restore();
    result
}

struct TuneIn {
    dds_config: [dds_data::DdsData; 10],
}

impl TuneIn {
    fn new() -> Self {
        Self {
            dds_config: [DdsData::new(); 10],
        }
    }

    fn cycle_sin_data(dds: &mut dds_data::DdsData) {
        let cycled_item = dds.signal_data[0].1;

        for i in 0..dds.signal_data.len() - 1 {
            dds.signal_data[i].1 = dds.signal_data[i + 1].1;
        }

        dds.signal_data[dds.signal_data.len() - 1].1 = cycled_item;
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(10);
        let mut last_tick = Instant::now();

        self.dds_config[0].enable_signal();

        loop {
            let _ = terminal.draw(|frame| self.draw(frame));
            if event::poll(tick_rate)? {
                if matches!(event::read()?, Event::Key(_)) {
                    break Ok(());
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        for i in 0..self.dds_config.len() - 1 {
            if self.dds_config[i].on && self.dds_config[i].last_cycle + Duration::from_millis(10) < Instant::now() {
                self.dds_config[i].last_cycle = Instant::now();
                Self::cycle_sin_data(&mut self.dds_config[i]);
            }
        }
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

    fn draw(&self, frame: &mut Frame) {
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

        let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().white())
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
                .data(&self.dds_config[i].signal_data);

            let chart = Chart::new(vec![dataset.clone()])
                .block(Block::new().borders(Borders::ALL))
                .x_axis(Axis::default().bounds([0.0, 100.0]))
                .y_axis(Axis::default().bounds([-5.0, 5.0]));

            frame.render_widget(chart, split_layout[0]);
            frame.render_widget(table.clone(), split_layout[1]);
        }
    }
}
