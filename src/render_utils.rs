use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, LineGauge, Padding, Paragraph,
        Row, Table, Widget,
    },
};
// Render the Main part of the application
pub fn render_general(
    frame: &mut Frame,
    layout: Rc<[Rect]>,
    serial: Table,
    current_strength: f64,
    current_octave: i32,
) {
    // Create the block surrounding signal info
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default())
            .style(Style::default())
            .title("Info")
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            }),
        layout[0],
    );

    let signal_info_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .horizontal_margin(2)
        .constraints(vec![Constraint::Length(3), Constraint::Length(3)])
        .split(layout[0]);

    LineGauge::default()
        .block(
            Block::new()
                .borders(Borders::NONE)
                .title(Line::from("Strength (0 <-> 255)").centered()),
        )
        .filled_style(Style::default().fg(Color::Blue).bg(Color::Blue))
        .unfilled_style(Style::default().fg(Color::Red).bg(Color::Red))
        .label(format!("{:0>3}", current_strength))
        .line_set(symbols::line::NORMAL)
        .ratio(current_strength / 255.)
        .render(signal_info_layout[0], frame.buffer_mut());

    LineGauge::default()
        .block(
            Block::new()
                .borders(Borders::NONE)
                .title(Line::from("Octave (-6 <-> 4)").centered()),
        )
        .filled_style(Style::default().fg(Color::Blue).bg(Color::Blue))
        .unfilled_style(Style::default().fg(Color::Red).bg(Color::Red))
        .label(format!("{:>3}", current_octave))
        .line_set(symbols::line::NORMAL)
        .ratio((current_octave + 6) as f64 / 10.)
        .render(signal_info_layout[1], frame.buffer_mut());

    frame.render_widget(
        Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default())
            .style(Style::default())
            .title("Communication"),
        layout[1],
    );

    frame.render_widget(serial, layout[1]);
}
// Render the dds visualisation
pub fn render_dds(frame: &mut Frame, layout: Rc<[Rect]>, channel_data: &Vec<(f64, f64)>) {
    // Create the dataset for the fft graph
    let dds_dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .style(Style::new().fg(Color::Blue))
        .graph_type(GraphType::Bar)
        .data(channel_data);

    // Create fft widget and the block surrounding it
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
                .bounds([0., 255.])
                .labels(["0", "255"]),
        );

    frame.render_widget(chart, layout[0]);
}
// Render the current state of the different states
pub fn render_channels(frame: &mut Frame, layout: Rc<[Rect]>, channel_data: &Vec<(f64, f64)>) {
    for i in 0..layout.len() {
        // Set default values used if a channel is unassigned
        let mut signal_freq = 0.;
        let mut signal_strength = 0.;

        // Check if a channel has valid data in it
        if i < channel_data.len() {
            signal_freq = channel_data[i].0;
            signal_strength = channel_data[i].1;
        }

        // Convert numerical values to string
        let freq_str = &format!("{:.1} Hz", signal_freq);
        let strength_str = &format!("{:.1}", signal_strength);

        // Create data rows
        let rows = [
            Row::new(vec!["Freq", freq_str]),
            Row::new(vec!["Strength", strength_str]),
        ];

        // Define how wide cells of table are
        let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];

        // Create table and the block surrounding it
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().white())
            .block(
                Block::new()
                    .border_type(BorderType::Thick)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .style(Style::default())
                    .title("Channel ".to_string() + &(i + 1).to_string())
                    .padding(Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 0,
                    }),
            );

        frame.render_widget(table.clone(), layout[i]);
    }
}
// Render shortcuts
pub fn render_shortcuts(frame: &mut Frame, layout: Rc<[Rect]>) {
    let shortcuts = "Quit: q | \
                    Com Config: p | \
                    Play tone: Home row | \
                    --Strength: v | \
                    ++Strength: V | \
                    Toggle random Strength: r | \
                    --Octave: n | \
                    ++Octave: N | \
                    Clear notes: c";
    let paragraph = Paragraph::new(shortcuts).style(Style::new().blue());
    frame.render_widget(paragraph, layout[1]);
}
