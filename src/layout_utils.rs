use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};
// Generate the main Layout
pub fn generate_main_layout(frame: &mut Frame) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
    // Base layer
    let base_layer = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());
    let vertical_temp_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(base_layer[0]);

    let general_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50); 2])
        .split(vertical_temp_layout[0]);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50); 2])
        .split(vertical_temp_layout[1]);

    let channel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50); 2])
        .split(main_layout[1]);

    let mut upper_channel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(channel_layout[0])
        .to_vec();

    let mut lower_channel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(channel_layout[1])
        .to_vec();

    let mut channel_layout = Vec::<Rect>::new();
    channel_layout.append(&mut upper_channel_layout);
    channel_layout.append(&mut lower_channel_layout);

    return (base_layer, general_layout, main_layout, channel_layout.into());
}
