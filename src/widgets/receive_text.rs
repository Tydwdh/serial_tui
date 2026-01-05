use ratatui::widgets::Widget;
use ratatui::{prelude::*, style::Color, widgets::*};
#[derive(Default)]
pub struct ReceiveTextState {
    content: String,
    scroll: u16,
}
impl ReceiveTextState {
    pub fn append_text(&mut self, str: &str) {
        self.content += str;
    }
}

pub struct ReceiveText;
impl StatefulWidget for ReceiveText {
    type State = ReceiveTextState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .border_style(Style::new().fg(Color::Gray))
            .title("接收区");
        let mut p = Paragraph::new(state.content.clone())
            .wrap(Wrap { trim: true })
            .block(block);
        let len = p.line_count(area.width - 2) as u16;
        let height = area.height + state.scroll;

        if len != height {
            state.scroll = len.saturating_sub(height);
        }
        p = p.scroll((state.scroll, 0));

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new((len).saturating_sub(area.height).into())
            .position(state.scroll.into());
        // println!("{len} {}", state.scroll);
        p.render(area, buf);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }
}
