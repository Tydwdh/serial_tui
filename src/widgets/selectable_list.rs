use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, StatefulWidget},
};

#[derive(Default)]
pub struct SelectableListState {
    items: Vec<String>,
    state: ListState,
    is_focus: bool,
}

impl SelectableListState {
    pub fn new(items: Vec<String>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self {
            items,
            state,
            is_focus: false,
        }
    }

    // 更新列表（比如刷新串口）
    pub fn update_items(&mut self, new_items: Vec<String>) {
        self.items = new_items;
    }

    // 处理向下
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = self.state.selected().unwrap_or(0);
        let next = (i + 1) % self.items.len();
        self.state.select(Some(next));
    }

    // 处理向上
    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = self.state.selected().unwrap_or(0);
        let next = (i + self.items.len() - 1) % self.items.len();
        self.state.select(Some(next));
    }

    // 获取当前选中的索引
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    // 获取当前选中的值
    pub fn selected_item(&self) -> Option<&str> {
        self.selected()
            .and_then(|i| self.items.get(i).map(|s| s.as_str()))
    }

    // 获取内部状态（用于 render_stateful_widget）
    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    // 只读访问 items
    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focus = focus;
    }
}

#[derive(Default)]
pub struct SelectableList {
    name: String,
}
impl SelectableList {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl StatefulWidget for SelectableList {
    type State = SelectableListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let items: Vec<ListItem> = state
            .items()
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect();

        let block = if state.is_focus {
            Block::bordered()
                .border_style(Style::default().fg(Color::Yellow))
                .title(self.name)
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
        } else {
            Block::bordered()
                .title(self.name)
                .border_style(Style::default().fg(Color::White)) // 非聚焦时灰色边框
        };

        let mut list = List::new(items).block(block);

        if state.is_focus {
            list = list.highlight_symbol(">").highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::White)
                    .fg(Color::Black),
            );
        }

        list.render(area, buf, state.state());
    }
}
