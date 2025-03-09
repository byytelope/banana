use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::canvas::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::WidgetRef,
};

#[derive(Default, Debug, Clone)]
pub struct Input {
    pub value: String,
    pub cursor: usize,
}

impl Input {
    pub fn new(text: String) -> Self {
        Self {
            value: text.clone(),
            cursor: text.chars().count(),
        }
    }

    pub fn handle_event(&mut self, event: &KeyEvent) {
        if !(event.kind == KeyEventKind::Press || event.kind == KeyEventKind::Repeat) {
            return;
        }

        match (event.code, event.modifiers) {
            // Insert character
            (KeyCode::Char(c), KeyModifiers::SHIFT | KeyModifiers::NONE) => {
                if self.cursor == self.value.chars().count() {
                    self.value.push(c);
                } else {
                    self.value = self
                        .value
                        .chars()
                        .take(self.cursor)
                        .chain(std::iter::once(c).chain(self.value.chars().skip(self.cursor)))
                        .collect();
                }

                self.cursor += 1;
            }

            // Delete previous character
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value = self
                        .value
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();
                }
            }

            // Delete next character
            (KeyCode::Delete, KeyModifiers::NONE) => {
                if self.cursor == self.value.chars().count() {
                    self.value = self
                        .value
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
                if self.cursor != self.value.chars().count() {
                    self.cursor += 1;
                }
            }

            // Delete previous word
            (KeyCode::Backspace, KeyModifiers::CONTROL) => {
                if self.cursor > 0 {
                    let remaining = self.value.chars().skip(self.cursor);
                    let rev = self
                        .value
                        .chars()
                        .rev()
                        .skip(self.value.chars().count().max(self.cursor) - self.cursor)
                        .skip_while(|c| !c.is_alphanumeric())
                        .skip_while(|c| c.is_alphanumeric())
                        .collect::<Vec<char>>();
                    let rev_len = rev.len();
                    self.value = rev.into_iter().rev().chain(remaining).collect();
                    self.cursor = rev_len;
                }
            }

            // Delete next word
            (KeyCode::Delete, KeyModifiers::CONTROL) => {
                if self.cursor != self.value.chars().count() {
                    self.value = self
                        .value
                        .chars()
                        .take(self.cursor)
                        .chain(
                            self.value
                                .chars()
                                .skip(self.cursor)
                                .skip_while(|c| c.is_alphanumeric())
                                .skip_while(|c| !c.is_alphanumeric()),
                        )
                        .collect();
                }
            }

            // Go to previous word
            (KeyCode::Left, KeyModifiers::CONTROL) => {
                if self.cursor > 0 {
                    self.cursor = self
                        .value
                        .chars()
                        .rev()
                        .skip(self.value.chars().count().max(self.cursor) - self.cursor)
                        .skip_while(|c| !c.is_alphanumeric())
                        .skip_while(|c| c.is_alphanumeric())
                        .count();
                }
            }

            // Go to next word
            (KeyCode::Right, KeyModifiers::CONTROL) => {
                if self.cursor != self.value.chars().count() {
                    self.cursor = self
                        .value
                        .chars()
                        .enumerate()
                        .skip(self.cursor)
                        .skip_while(|(_, c)| c.is_alphanumeric())
                        .find(|(_, c)| c.is_alphanumeric())
                        .map(|(i, _)| i)
                        .unwrap_or_else(|| self.value.chars().count());
                }
            }

            _ => {}
        }
    }
}

impl WidgetRef for Input {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text = Paragraph::new(&*self.value)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                    .border_style(Style::new().light_yellow()),
            )
            .centered();
        text.render(area, buf);
    }
}
