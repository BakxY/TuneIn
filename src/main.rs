use crossterm::event::{self, Event};
use dds_data::DdsData;
use ratatui::{
    style::{Color, Style}, symbols, widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Padding}, DefaultTerminal, Frame
};
use std::{
    io::Result,
    time::{Duration, Instant},
};

pub mod dds_data;
pub mod layout_utils;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = TuneIn::new().run(terminal);
    ratatui::restore();
    result
}

struct TuneIn {
    dds_config: dds_data::DdsData,
}

impl TuneIn {
    fn new() -> Self {
        Self {
            dds_config: DdsData::new(),
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(1);
        let mut last_tick = Instant::now();

        self.dds_config.add_signal(5., 1.);
        self.dds_config.add_signal(1500., 3.);
        self.dds_config.add_signal(3000., 0.);
        self.dds_config.add_signal(4500., 2.);
        self.dds_config.add_signal(6000., 1.);

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

    fn on_tick(&mut self) {}

    fn draw(&self, frame: &mut Frame) {
        let (general_layout, fft_layout, channel_layout) = layout_utils::generate_main_layout(frame);

        frame.render_widget(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default())
                .style(Style::default())
                .title("Info"),
            general_layout[0],
        );
        frame.render_widget(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default())
                .style(Style::default())
                .title("Options"),
            general_layout[1],
        );
        frame.render_widget(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default())
                .style(Style::default())
                .title("Communication"),
            general_layout[2],
        );

        let dds_dataset = Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::new().fg(Color::Blue))
            .graph_type(GraphType::Bar)
            .data(&self.dds_config.signal_data);

        let chart = Chart::new(vec![dds_dataset.clone()])
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title("FFT")
                    .padding(Padding::new(1, 4, 1, 1))
            )
            .x_axis(
                Axis::default()
                    .title("Frequency")
                    .style(Style::default().fg(Color::White))
                    .bounds([0., 6000.])
                    .labels(["0", "3000", "6000"]),
            )
            .y_axis(
                Axis::default()
                    .title("Strength")
                    .style(Style::default().fg(Color::White))
                    .bounds([0., 10.])
                    .labels(["0", "10"])
            );

        frame.render_widget(chart, fft_layout);

        for i in 0..channel_layout.len() {
            let root_block = Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default())
                .title("Channel ".to_string() + &(i + 1).to_string());

            frame.render_widget(root_block.clone(), channel_layout[i]);

            /*let freq_str = &format!("{:.2} Hz", self.dds_config[i].freq);
            let attenu_str = &format!("{:.2}", self.dds_config[i].attenu);

            let rows = [
                Row::new(vec!["Note", "IDFK"]),
                Row::new(vec!["Freq", freq_str]),
                Row::new(vec!["Atten", attenu_str]),
            ];

            let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
            let table = Table::new(rows, widths)
                .column_spacing(1)
                .style(Style::new().white())
                .block(Block::new());

            frame.render_widget(table.clone(), split_layout[1]);*/
        }
    }
}
