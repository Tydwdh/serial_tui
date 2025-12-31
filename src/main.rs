use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout, Position},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
mod selectable_list;
use selectable_list::*;
mod text_input;
use text_input::*;
mod command;
use command::*;

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

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut mode = Mode::CommandInput;
    // 初始化状态：选中第一个串口（如果存在）
    let mut uart_list: Vec<String> = vec![];
    if let Ok(ports) = serialport::available_ports() {
        uart_list = ports.iter().map(|p| p.port_name.clone()).collect();
    }
    let mut uart_list_state = SelectableListState::new(uart_list);

    let bartrata_list = vec!["460800".to_string(), "500000".to_string()];
    let mut bartrata_list_state = SelectableListState::new(bartrata_list);

    const TIMEOUT: Duration = Duration::from_millis(100);
    let mut text_input_state = TextInputState::default();

    loop {
        let mut uart_list: Vec<String> = vec![];
        if let Ok(ports) = serialport::available_ports() {
            uart_list = ports.iter().map(|p| p.port_name.clone()).collect();
            uart_list_state.update_items(uart_list);
        }
        // 处理事件
        if event::poll(TIMEOUT)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match mode {
                Mode::UartChoice => match key.code {
                    KeyCode::Down => {
                        uart_list_state.next();
                    }
                    KeyCode::Up => {
                        uart_list_state.previous();
                    }
                    KeyCode::Enter => mode = Mode::CommandInput,
                    _ => {}
                },
                Mode::CommandInput => {
                    if let Ok(command) = parse_command(&text_input_state.handle_key(key)) {
                        match command {
                            Command::ModeToUartChoice => mode = Mode::UartChoice,
                            Command::Quit => return Ok(()),
                            Command::ModeToRateChoice => mode = Mode::RateChoice,
                        }
                    }
                }
                Mode::RateChoice => match key.code {
                    KeyCode::Down => {
                        bartrata_list_state.next();
                    }
                    KeyCode::Up => {
                        bartrata_list_state.previous();
                    }
                    KeyCode::Enter => mode = Mode::CommandInput,
                    _ => {}
                },
            }
            if let KeyCode::Char(':') = key.code {
                mode = Mode::CommandInput;
            }
        }
        terminal.draw(|frame| {
            let mut layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
            let [up, command] = frame.area().try_layout(&layout).unwrap();
            layout = Layout::horizontal([Constraint::Length(10), Constraint::Fill(1)]);

            let [left, receive_data] = up.try_layout(&layout).unwrap();

            if mode == Mode::UartChoice {
                layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(0)]);
                *uart_list_state.state().offset_mut() = 0;
                uart_list_state.set_focus(true);
            } else if mode == Mode::RateChoice {
                layout = Layout::vertical([Constraint::Length(0), Constraint::Fill(1)]);
                *bartrata_list_state.state().offset_mut() = 0;
                bartrata_list_state.set_focus(true);
            } else {
                layout = Layout::vertical([Constraint::Length(3), Constraint::Length(3)]);
                uart_list_state.set_focus(false);
                bartrata_list_state.set_focus(false);
            }

            if mode == Mode::CommandInput {
                text_input_state.set_focus(true);
            } else {
                text_input_state.set_focus(false);
            }

            let [uart_list_area, bartrate_list_area] = left.try_layout(&layout).unwrap();
            frame.render_stateful_widget(
                SelectableList::new("端口号".to_string()),
                uart_list_area,
                &mut uart_list_state,
            );
            frame.render_stateful_widget(
                SelectableList::new("波特率".to_string()),
                bartrate_list_area,
                &mut bartrata_list_state,
            );
            frame.render_stateful_widget(TextInput, command, &mut text_input_state);
        })?;
    }
}
