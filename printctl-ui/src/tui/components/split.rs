use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Buffer, Widget},
    widgets::WidgetRef,
};

#[derive(Default)]
pub struct SplitLayout {
    direction: Direction,
    items: Vec<Box<dyn WidgetRef>>,
}

impl SplitLayout {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            items: Vec::default(),
        }
    }
    pub fn direction(self, direction: Direction) -> Self {
        Self {
            direction,
            items: self.items,
        }
    }

    pub fn item<W>(mut self, item: W) -> Self
    where
        W: WidgetRef + 'static,
    {
        self.items.push(Box::new(item));
        self
    }

    fn layout(&self, area: Rect) -> Vec<Rect> {
        if self.items.is_empty() {
            return vec![];
        }

        let constraints = vec![Constraint::Ratio(1, self.items.len() as u32); self.items.len()];

        Layout::default()
            .direction(self.direction)
            .constraints(constraints)
            .split(area)
            .to_vec()
    }
}

impl Widget for SplitLayout {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = self.layout(area).into_iter();
        for (widget, rect) in self.items.iter().zip(areas) {
            widget.render_ref(rect, buf);
        }
    }
}
