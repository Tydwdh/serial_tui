use super::*;

#[derive(Debug, Clone)]
pub enum Action {
    None,
    Quit,
    SwitchMode(Mode),   // 切换当前焦点模式
    SelectPort(String), // 选中了某个串口
    SelectRate(String), // 选中了某个波特率
    Error(String),
}
