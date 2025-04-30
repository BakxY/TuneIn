use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, List, ListDirection},
};
use std::{io::Result, rc::Rc};

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

fn split_midi_layout(layout: Rect) -> Vec<Rect> {
    let split_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(2)
        .split(layout)
        .to_vec();

    return split_layout;
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
        .marker(symbols::Marker::Braille)
        .style(Style::new().fg(Color::Red))
        .graph_type(GraphType::Line)
        .data(&[
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
            (100.0, 0.),
        ]);

    let list = List::new(["> Tone", "> Frequency", "> Velocity"])
        .style(Style::new().white())
        .direction(ListDirection::TopToBottom);

    for i in 0..midi_layout.len() {
        let root_block = Block::new()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default())
            .title("DDS ".to_string() + &(i + 1).to_string());

        frame.render_widget(root_block.clone(), midi_layout[i]);

        let split_layout = split_midi_layout(midi_layout[i]);

        let chart = Chart::new(vec![dataset.clone()])
            .block(Block::new().borders(Borders::ALL))
            .x_axis(Axis::default().bounds([0.0, 100.0]))
            .y_axis(Axis::default().bounds([-5.0, 5.0]));

        frame.render_widget(chart, split_layout[0]);
        frame.render_widget(list.clone(), split_layout[1]);
    }
}
