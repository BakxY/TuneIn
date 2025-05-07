use crossterm::event::{self, Event, KeyCode};
use dds_data::DdsData;
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Padding},
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

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = TuneIn::new().run(terminal);
    ratatui::restore();
    result
}

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
            state: AppState::ComConfig,
            dds_config: DdsData::new(),
            com_config: ComConfig::new(),
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
            let _ = terminal.draw(|frame| self.draw_running(frame));

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    match self.state {
                        AppState::Running => match key.code {
                            KeyCode::Char('q') => break Ok(()),
                            KeyCode::Char('p') => self.state = AppState::ComConfig,
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

    fn draw_running(&mut self, frame: &mut Frame) {
        let (general_layout, fft_layout, channel_layout) =
            layout_utils::generate_main_layout(frame);

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
                    .padding(Padding::new(1, 4, 1, 1)),
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
                    .labels(["0", "10"]),
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

    fn draw_com_config(&mut self, frame: &mut Frame) {
        self.com_config.scan_serialports();
        self.com_config.show_com_popup(frame);
    }
}
