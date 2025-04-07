use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::WidgetRef,
};

#[derive(Default, Debug, Clone)]
pub struct Input {
    pub buffer: String,
    pub text: String,
    pub cursor: usize,
}

impl Input {
    pub fn new(text: String) -> Self {
        Self {
            buffer: String::new(),
            text: text.clone(),
            cursor: 0,
        }
    }

    pub fn handle_event(&mut self, event: &KeyEvent) {
        if !(event.kind == KeyEventKind::Press || event.kind == KeyEventKind::Repeat) {
            return;
        }

        match (event.code, event.modifiers) {
            // Insert character
            (KeyCode::Char(c), KeyModifiers::SHIFT | KeyModifiers::NONE) => {
                if self.cursor == self.buffer.chars().count() {
                    self.buffer.push(c);
                } else {
                    self.buffer = self
                        .buffer
                        .chars()
                        .take(self.cursor)
                        .chain(std::iter::once(c).chain(self.buffer.chars().skip(self.cursor)))
                        .collect();
                }

                self.cursor += 1;
            }

            // Delete previous character
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer = self
                        .buffer
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();
                }
            }

            // Delete next character
            (KeyCode::Delete, KeyModifiers::NONE) => {
                if self.cursor == self.buffer.chars().count() {
                    self.buffer = self
                        .buffer
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();
                }
            }

            // Go to previous character
            (KeyCode::Left, KeyModifiers::NONE) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            // Go to next character
            (KeyCode::Right, KeyModifiers::NONE) => {
                if self.cursor != self.buffer.chars().count() {
                    self.cursor += 1;
                }
            }

            // Delete previous word
            (KeyCode::Backspace, KeyModifiers::ALT) => {
                if self.cursor > 0 {
                    let remaining = self.buffer.chars().skip(self.cursor);
                    let rev = self
                        .buffer
                        .chars()
                        .rev()
                        .skip(self.buffer.chars().count().max(self.cursor) - self.cursor)
                        .skip_while(|c| !c.is_alphanumeric())
                        .skip_while(|c| c.is_alphanumeric())
                        .collect::<Vec<char>>();
                    let rev_len = rev.len();
                    self.buffer = rev.into_iter().rev().chain(remaining).collect();
                    self.cursor = rev_len;
                }
            }

            // Delete next word
            (KeyCode::Delete, KeyModifiers::ALT) => {
                if self.cursor != self.buffer.chars().count() {
                    self.buffer = self
                        .buffer
                        .chars()
                        .take(self.cursor)
                        .chain(
                            self.buffer
                                .chars()
                                .skip(self.cursor)
                                .skip_while(|c| c.is_alphanumeric())
                                .skip_while(|c| !c.is_alphanumeric()),
                        )
                        .collect();
                }
            }

            // Delete line
            (KeyCode::Backspace, KeyModifiers::SUPER) => {
                self.cursor = 0;
                self.buffer.clear();
            }

            // Go to previous word
            (KeyCode::Left, KeyModifiers::ALT) => {
                if self.cursor > 0 {
                    self.cursor = self
                        .buffer
                        .chars()
                        .rev()
                        .skip(self.buffer.chars().count().max(self.cursor) - self.cursor)
                        .skip_while(|c| !c.is_alphanumeric())
                        .skip_while(|c| c.is_alphanumeric())
                        .count();
                }
            }

            // Go to next word
            (KeyCode::Right, KeyModifiers::ALT) => {
                if self.cursor != self.buffer.chars().count() {
                    self.cursor = self
                        .buffer
                        .chars()
                        .enumerate()
                        .skip(self.cursor)
                        .skip_while(|(_, c)| c.is_alphanumeric())
                        .find(|(_, c)| c.is_alphanumeric())
                        .map(|(i, _)| i)
                        .unwrap_or_else(|| self.buffer.chars().count());
                }
            }

            // Go to start
            (KeyCode::Left, KeyModifiers::SUPER) => {
                self.cursor = 0;
            }

            // Go to end
            (KeyCode::Right, KeyModifiers::SUPER) => {
                self.cursor = self.buffer.chars().count();
            }

            _ => {}
        }
    }
}

impl WidgetRef for Input {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text = Paragraph::new(&*self.text)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                    .border_style(Style::new().light_yellow()),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        text.render(area, buf);
    }
}
