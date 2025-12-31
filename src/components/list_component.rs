use crate::widgets::{SelectableList, SelectableListState};

use super::*;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

pub struct ListComponent {
    pub state: SelectableListState,
    title: String,
    // 这是一个回调或者标记，用来区分是串口列表还是波特率列表
    on_select: fn(String) -> Action,
}

impl ListComponent {
    pub fn new(title: String, items: Vec<String>, on_select: fn(String) -> Action) -> Self {
        Self {
            state: SelectableListState::new(items),
            title,
            on_select,
        }
    }
}

impl Component for ListComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        match key.code {
            KeyCode::Down => self.state.next(),
            KeyCode::Up => self.state.previous(),
            KeyCode::Enter => {
                if let Some(item) = self.state.selected_item() {
                    return Ok((self.on_select)(item.to_string()));
                }
            }
            // 可以在这里处理 Esc 返回默认模式等逻辑
            _ => {}
        }
        Ok(Action::None)
    }

    fn render(&mut self, f: &mut Frame, area: Rect, is_active: bool) {
        self.state.set_focus(is_active);
        let widget = SelectableList::new(self.title.clone());
        f.render_stateful_widget(widget, area, &mut self.state);
    }
}
