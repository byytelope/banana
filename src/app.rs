use std::error;

use crate::input::Input;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub input: Input,
    pub word_index: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: Input::new("the quick brown fox jumps over the lazy dog".to_string()),
            word_index: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
