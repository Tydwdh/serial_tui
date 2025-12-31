use crossterm::event::KeyEvent;

pub trait EventHandler {
    /// 处理按键事件，返回是否消费了该事件（true = 不再传递）
    fn handle_key_event(&mut self, key: KeyEvent) -> bool;
}
