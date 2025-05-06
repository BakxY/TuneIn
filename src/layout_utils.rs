use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn generate_main_layout(frame: &mut Frame) -> (Rc<[Rect]>, Rc<[Rect]>) {
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

    let midi_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(66), Constraint::Percentage(34)])
        .split(vertical_temp_layout[1]);

    return (general_layout, midi_layout);
}

pub fn split_midi_layout(layout: Rect) -> Vec<Rect> {
    let split_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(2)
        .split(layout)
        .to_vec();

    return split_layout;
}
