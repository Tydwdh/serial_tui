use crate::action::Action;
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
mod list_component;
pub use list_component::*;
mod input_component;
pub use input_component::*;
pub trait Component {
    // 处理按键，返回一个 Action 告诉 App 该做什么
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action>;

    // 渲染自己
    // is_active 用于判断是否需要高亮边框
    fn render(&mut self, f: &mut Frame, area: Rect, is_active: bool);
}
