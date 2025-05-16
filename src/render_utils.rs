use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Padding, Row, Table},
};

pub fn render_general(
    frame: &mut Frame,
    layout: Vec<Rect>,
    current_attenu: f64,
    current_octave: i32,
) {
    // Convert numerical values to string
    let attenu_str = &format!("{}", current_attenu);
    let octave_str = &format!("{}", current_octave);

    // Create data rows
    let rows = [
        Row::new(vec!["Current attenu", attenu_str]),
        Row::new(vec!["Current octave", octave_str]),
    ];

    // Define how wide cells of table are
    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

    // Create table and the block surrounding it
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().white())
        .block(
            Block::new()
                .border_type(BorderType::Thick)
                .borders(Borders::ALL)
                .border_style(Style::default())
                .style(Style::default())
                .title("Info"),
        );

    frame.render_widget(table.clone(), layout[0]);

    frame.render_widget(
        Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default())
            .style(Style::default())
            .title("Communication"),
        layout[1],
    );
}

pub fn render_dds(frame: &mut Frame, layout: Rect, channel_data: &Vec<(f64, f64)>) {
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
                .bounds([0., 10.])
                .labels(["0", "10"]),
        );

    frame.render_widget(chart, layout);
}

pub fn render_channels(frame: &mut Frame, layout: Vec<Rect>, channel_data: &Vec<(f64, f64)>) {
    for i in 0..layout.len() {
        // Set default values used if a channel is unassigned
        let mut signal_freq = 0.;
        let mut signal_strength = 10.;

        // Check if a channel has valid data in it
        if i < channel_data.len() {
            signal_freq = channel_data[i].0;
            signal_strength = channel_data[i].1;
        }

        // Convert numerical values to string
        let freq_str = &format!("{:.1} Hz", signal_freq);
        let attenu_str = &format!("{:.1}", 10. - signal_strength);

        // Create data rows
        let rows = [
            Row::new(vec!["Freq", freq_str]),
            Row::new(vec!["Atten", attenu_str]),
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
