use super::*;
use crossterm::event::KeyCode::*;
use ratatui::widgets::Widget;
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::*,
};
#[derive(Debug, Default, Clone)]
pub struct TextInputState {
    content: String,
    cursor: usize, // 光标位置（0 <= cursor <= content.len()）
    is_focus: bool,
}

impl TextInputState {
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> String {
        if key.kind != crossterm::event::KeyEventKind::Press {
            return String::new();
        }

        match key.code {
            crossterm::event::KeyCode::Char(ch) => {
                // ✅ 将字符位置转换为字节位置
                let byte_pos = self
                    .content
                    .char_indices()
                    .nth(self.cursor)
                    .map(|(i, _)| i)
                    .unwrap_or(self.content.len());

                self.content.insert(byte_pos, ch);
                self.cursor += 1; // 字符位置 +1
            }
            crossterm::event::KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    // 找到前一个字符的字节位置
                    let byte_pos = self
                        .content
                        .char_indices()
                        .nth(self.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    // 删除一个字符（可能多字节）
                    let next_byte = self
                        .content
                        .char_indices()
                        .nth(self.cursor + 1)
                        .map(|(i, _)| i)
                        .unwrap_or(self.content.len());
                    self.content.drain(byte_pos..next_byte);
                }
            }
            crossterm::event::KeyCode::Delete => {
                if self.cursor < self.content.chars().count() {
                    let byte_pos = self
                        .content
                        .char_indices()
                        .nth(self.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(self.content.len());
                    let next_byte = self
                        .content
                        .char_indices()
                        .nth(self.cursor + 1)
                        .map(|(i, _)| i)
                        .unwrap_or(self.content.len());
                    self.content.drain(byte_pos..next_byte);
                }
            }
            crossterm::event::KeyCode::Left => {
                self.cursor = self.cursor.saturating_sub(1);
            }
            crossterm::event::KeyCode::Right => {
                self.cursor = (self.cursor + 1).min(self.content.chars().count());
            }
            crossterm::event::KeyCode::Home => self.cursor = 0,
            crossterm::event::KeyCode::End => self.cursor = self.content.chars().count(),
            crossterm::event::KeyCode::Esc => return String::new(),
            crossterm::event::KeyCode::Enter => {
                let s = self.content.clone();
                self.content.clear();
                self.cursor = 0;
                return s;
            }
            _ => {}
        }
        String::new()
    }

    pub fn value(&self) -> &str {
        &self.content
    }

    pub fn set_value(&mut self, s: String) {
        self.content = s;
        self.cursor = self.content.chars().count(); // ✅ 字符数，不是字节数
    }

    pub fn visual_cursor(&self) -> usize {
        self.cursor // 字符位置，正确
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focus = focus;
    }
}

pub struct TextInput;

impl StatefulWidget for TextInput {
    type State = TextInputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        use ratatui::style::{Modifier, Style};
        use ratatui::text::{Line, Span};

        let block = if state.is_focus {
            Block::bordered()
                .border_style(Style::new().fg(Color::Yellow))
                .title("Input")
        } else {
            Block::bordered()
                .border_style(Style::new().fg(Color::Gray))
                .title("Input")
        };

        let text_line = if state.is_focus {
            // === 只有聚焦时才显示模拟光标 ===
            let cursor_style = Style::new().bg(Color::White).fg(Color::Black);
            let chars: Vec<char> = state.value().chars().collect();
            let cursor = state.cursor.min(chars.len());

            if cursor == chars.len() {
                // 光标在末尾
                Line::from(vec![
                    Span::raw(state.value()),
                    Span::styled(" ", cursor_style),
                ])
            } else {
                // 光标在中间
                let before: String = chars[..cursor].iter().collect();
                let current = chars[cursor];
                let after: String = chars[cursor + 1..].iter().collect();
                Line::from(vec![
                    Span::raw(before),
                    Span::styled(current.to_string(), cursor_style),
                    Span::raw(after),
                ])
            }
        } else {
            // === 非聚焦：纯文本，无光标 ===
            Line::from(Span::raw(state.value()))
        };

        Paragraph::new(text_line)
            .block(block)
            .style(Style::new().fg(Color::LightBlue))
            .render(area, buf);
    }
}
