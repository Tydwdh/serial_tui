use crate::{
    command::*,
    widgets::{TextInput, TextInputState},
};

use super::*;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

pub struct CommandInputComponent {
    state: TextInputState,
}

impl CommandInputComponent {
    pub fn new() -> Self {
        Self {
            state: TextInputState::default(),
        }
    }
}

impl Component for CommandInputComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        // 先让 state 处理输入
        let input_result = self.state.handle_key(key);

        // 如果按下了 Enter，TextInputState 会返回非空字符串
        if !input_result.is_empty() {
            match parse_command(&input_result) {
                Ok(Command::ModeToUartChoice) => {
                    return Ok(Action::SwitchMode(crate::Mode::UartChoice));
                }
                Ok(Command::ModeToRateChoice) => {
                    return Ok(Action::SwitchMode(crate::Mode::RateChoice));
                }
                Ok(Command::Quit) => return Ok(Action::Quit),
                Err(e) => return Ok(Action::Error(e)),
            }
        }

        // 特殊按键处理，比如按下 ':' 聚焦（如果需要的话，或者在 App 层处理）
        Ok(Action::None)
    }

    fn render(&mut self, f: &mut Frame, area: Rect, is_active: bool) {
        self.state.set_focus(is_active);
        f.render_stateful_widget(TextInput, area, &mut self.state);
    }
}
