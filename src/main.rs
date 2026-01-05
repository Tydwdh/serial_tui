use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};
mod action;
mod command;
mod widgets;
use action::*;
mod components;
use components::*;
use serialport::SerialPort;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    UartChoice,
    RateChoice,
    CommandInput,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}
struct App {
    com: String,
    rate: u32,
    port: Option<Box<dyn SerialPort>>,
    should_quit: bool,
    mode: Mode,
    // 实例化组件
    uart_list: ListComponent,
    rate_list: ListComponent,
    input: CommandInputComponent,
    receive_area: ReceiveComponent,
}

impl App {
    fn new() -> Self {
        // 初始化逻辑
        let ports: Vec<String> = serialport::available_ports()
            .map(|p| p.iter().map(|x| x.port_name.clone()).collect())
            .unwrap_or_default();

        Self {
            com: ports[0].clone(),
            rate: 9600,
            port: None,
            should_quit: false,
            mode: Mode::CommandInput, // 默认模式
            uart_list: ListComponent::new(
                "端口号".to_string(),
                ports,
                Action::SelectPort, // 闭包：决定选中后产生什么 Action
            ),
            rate_list: ListComponent::new(
                "波特率".to_string(),
                vec!["9600".into(), "115200".into()],
                Action::SelectRate,
            ),
            input: CommandInputComponent::new(),
            receive_area: ReceiveComponent::new(),
        }
    }

    // 统一更新逻辑
    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::SwitchMode(mode) => self.mode = mode,
            Action::SelectPort(port) => {
                self.com = port.clone();
                self.mode = Mode::CommandInput;
            }
            Action::SelectRate(rate) => {
                // 设置波特率逻辑
                self.rate = rate.parse().unwrap();
                self.mode = Mode::CommandInput;
            }
            Action::Error(e) => {
                // 可以在这里设置一个错误提示弹窗的状态
            }
            Action::None => {}
            Action::Open => {
                self.port = serialport::new(self.com.clone(), self.rate)
                    .timeout(Duration::from_millis(1)) // 超时设置
                    .open()
                    .ok();
            }
        }
    }

    // 获取当前聚焦的组件
    fn get_active_component_mut(&mut self) -> &mut dyn Component {
        match self.mode {
            Mode::UartChoice => &mut self.uart_list,
            Mode::RateChoice => &mut self.rate_list,
            Mode::CommandInput => &mut self.input,
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // 1. 总体布局：上面是数据区，下面是命令输入
        // 你原本的代码在 UartChoice/RateChoice 时似乎想把 command 压缩？
        // 这里我根据你的逻辑还原：
        // 如果你需要根据模式改变底部输入框的高度，可以在这里 match self.mode
        let main_constraints = [Constraint::Fill(1), Constraint::Length(3)];

        let main_layout = Layout::vertical(main_constraints).split(area);
        let up_area = main_layout[0];
        let command_area = main_layout[1];

        // 2. 上半部分布局：左边是列表，右边是接收区
        let hor_layout = Layout::horizontal([
            Constraint::Length(20), // 左侧面板宽度
            Constraint::Fill(1),    // 右侧接收区
        ])
        .split(up_area);

        let left_panel_area = hor_layout[0];
        let receive_data_area = hor_layout[1]; // 暂时没用到，留给未来

        // 3. 左侧面板布局：这是你最关心的动态部分
        // 根据当前模式，决定 串口列表 和 波特率列表 的高度比例
        let left_constraints = match self.mode {
            Mode::UartChoice => {
                *self.uart_list.state.state().offset_mut() = 0;
                [
                    Constraint::Fill(1),   // 串口列表占满
                    Constraint::Length(0), // 波特率列表隐藏 (或者设为 Min(1) 显示一点点)
                ]
            }
            Mode::RateChoice => {
                *self.rate_list.state.state().offset_mut() = 0;
                [
                    Constraint::Length(0), // 串口列表隐藏
                    Constraint::Fill(1),   // 波特率列表占满
                ]
            }
            Mode::CommandInput => [
                Constraint::Length(3), // 默认均分，或者按需分配
                Constraint::Length(3),
            ],
        };

        let left_layout = Layout::vertical(left_constraints).split(left_panel_area);
        let uart_area = left_layout[0];
        let rate_area = left_layout[1];

        // 4. 渲染组件
        // render 方法签名：fn render(&mut self, f: &mut Frame, area: Rect, is_active: bool)

        // 渲染串口列表
        self.uart_list
            .render(frame, uart_area, self.mode == Mode::UartChoice);

        // 渲染波特率列表
        self.rate_list
            .render(frame, rate_area, self.mode == Mode::RateChoice);

        // 渲染输入框
        self.input
            .render(frame, command_area, self.mode == Mode::CommandInput);

        // 如果有接收区组件，也在这里渲染
        self.receive_area.render(frame, receive_data_area, false);
    }

    fn try_read_serial_data(&mut self) -> Result<()> {
        if let Some(port) = &mut self.port {
            let mut buffer = [0u8; 256]; // 一次最多读 256 字节
            match port.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buffer[..n]);
                    self.receive_area.state.append_text(&data);
                }
                Ok(_) => {} // 读到 0 字节（无数据）
                Err(e) => {
                    // 可能是超时（正常），也可能是断开
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        // 真实错误：串口断开？
                        eprintln!("串口读取错误: {}", e);
                        self.port = None; // 关闭串口
                    }
                }
            }
        }
        Ok(())
    }
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        app.try_read_serial_data().unwrap();

        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            // 全局快捷键处理 (比如 : 键)
            if let KeyCode::Esc = key.code {
                app.update(Action::SwitchMode(Mode::CommandInput));
                continue;
            }

            // 3. 将事件派发给当前活跃的组件
            let action = app.get_active_component_mut().handle_key_events(key)?;

            // 4. App 处理组件返回的 Action
            app.update(action);
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
