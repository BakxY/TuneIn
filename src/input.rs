use crossterm::event::KeyEvent;
use ratatui::{
    crossterm::event::{KeyCode, KeyEventKind},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};

/// Input holds the state of the user input
pub struct Input {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode
    input_mode: InputMode,
}

enum InputMode {
    Normal,
    Error,
    Editing,
}

impl Input {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn display_error(&mut self, message: String) {
        self.input = message;
        self.input_mode = InputMode::Error;
    }

    pub fn submit_message(&mut self) -> String {
        let tmp_input = self.input.clone();
        self.input.clear();
        self.reset_cursor();
        return tmp_input;
    }

    pub fn key_event(&mut self, key: KeyEvent) -> bool {
        let mut should_exit = false;
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    should_exit = true;
                }
                KeyCode::Char('i') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Backspace | KeyCode::Char('x') => self.delete_char(),
                KeyCode::Char('d') => {
                    self.input.clear();
                    self.reset_cursor();
                }
                _ => {}
            },
            InputMode::Error => {
                self.input_mode = InputMode::Editing;
                self.input.clear();
                self.reset_cursor();
            }
            InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            InputMode::Editing => {}
        };
        return should_exit;
    }

    pub fn get_input(&self, title: String) -> Paragraph<'_> {
        Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Error => Style::default().fg(Color::Red),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::bordered()
                    .title(title)
                    .border_type(BorderType::Thick),
            )
    }

    pub fn get_index(&self) -> u16 {
        self.character_index.try_into().unwrap()
    }
    pub fn get_shortcuts(&self) -> &str {
        match self.input_mode {
            InputMode::Editing => {
                "Normal Mode: Esc | \
                Submit: Enter | \
                Normal Text writing"
            }
            InputMode::Normal => {
                "Quit Config: q | \
                Input Mode: i | \
                Submit: Enter | \
                Delet Char: Backspace/x |\
                Clear input: d"
            }
            InputMode::Error => "Press any key",
        }
    }
}
