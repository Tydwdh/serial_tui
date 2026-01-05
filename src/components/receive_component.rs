use crate::{
    command::*,
    widgets::{TextInput, TextInputState},
};

use super::*;
use crate::widgets::*;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

pub struct ReceiveComponent {
    pub state: ReceiveTextState,
}
impl ReceiveComponent {
    pub fn new() -> Self {
        Self {
            state: ReceiveTextState::default(),
        }
    }
}

impl Component for ReceiveComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        Ok(Action::None)
    }
    fn render(&mut self, f: &mut Frame, area: Rect, is_active: bool) {
        f.render_stateful_widget(ReceiveText, area, &mut self.state);
    }
}
