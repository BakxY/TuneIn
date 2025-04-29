use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, symbols, text::Line, widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType}, DefaultTerminal, Frame
};
use std::{io::Result, ops::Index, rc::Rc};

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
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
            Constraint::Percentage(33),
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

fn render(frame: &mut Frame) {
    let (general_layout, midi_layout) = generate_layout(frame);

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

    let dataset = Dataset::default()
        .marker(symbols::Marker::Dot)
        .style(Style::new().fg(Color::Cyan))
        .graph_type(GraphType::Scatter)
        .data(&[
            (0., 0.),
            (10., 29.38),
            (20., 47.55),
            (30., 47.55),
            (40., 29.38),
            (50., 0.),
            (60., -29.38),
            (70., -47.55),
            (80., -47.55),
            (90., -29.38),
            (100., 0.),
        ]);

    for i in 0..midi_layout.len() {
        let block = Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default())
            .title("DDS ".to_string() + &(i + 1).to_string());

        frame.render_widget(block.clone(), midi_layout[i]);

        let chart = Chart::new(vec![dataset.clone()])
        .block(block)
        .x_axis(
            Axis::default()
                .style(Style::default().gray())
                .bounds([0.0, 100.0])
                //.labels(["0".bold(), "50".into(), "100.0".bold()]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().gray())
                .bounds([-50.0, 50.0])
                //.labels(["0".bold(), "50".into(), "100.0".bold()]),
        )
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        frame.render_widget(chart, midi_layout[i]);
    }
}
