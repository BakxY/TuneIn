use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn generate_main_layout(frame: &mut Frame) -> (Vec<Rect>, Rect, Vec<Rect>) {
    let vertical_temp_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());

    let general_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(vertical_temp_layout[0])
        .to_vec();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50); 2])
        .split(vertical_temp_layout[1]);

    let vertical_split_channel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50); 2])
        .split(main_layout[1]);

    let mut upper_channel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(vertical_split_channel_layout[0])
        .to_vec();

    let mut lower_channel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(vertical_split_channel_layout[1])
        .to_vec();

    let mut channel_layout = Vec::<Rect>::new();
    channel_layout.append(&mut upper_channel_layout);
    channel_layout.append(&mut lower_channel_layout);

    return (general_layout, main_layout[0], channel_layout);
}