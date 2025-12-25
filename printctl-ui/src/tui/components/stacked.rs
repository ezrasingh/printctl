use ratatui::{
    layout::Rect,
    prelude::{Buffer, Widget},
    widgets::{Block, Borders},
};

#[derive(Default)]
pub struct EmptyHeader;

#[derive(Default)]
pub struct EmptyLayout;

#[derive(Default)]
pub struct EmptyFooter;

#[derive(Default)]
pub struct StackedLayout<H, T, F> {
    header: H,
    content: T,
    footer: F,
}

impl StackedLayout<EmptyHeader, EmptyLayout, EmptyFooter> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<H, T, F> StackedLayout<H, T, F> {
    pub fn header<W>(self, widget: W) -> StackedLayout<W, T, F>
    where
        W: Widget,
    {
        StackedLayout {
            header: widget,
            content: self.content,
            footer: self.footer,
        }
    }

    pub fn content<W>(self, widget: W) -> StackedLayout<H, W, F>
    where
        W: Widget,
    {
        StackedLayout {
            header: self.header,
            content: widget,
            footer: self.footer,
        }
    }

    pub fn footer<W>(self, widget: W) -> StackedLayout<H, T, W>
    where
        W: Widget,
    {
        StackedLayout {
            header: self.header,
            content: self.content,
            footer: widget,
        }
    }

    fn layout(area: Rect) -> [Rect; 3] {
        let header_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        let footer_area = Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };

        let content_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: area.height.saturating_sub(2),
        };

        [header_area, content_area, footer_area]
    }
}

impl<H, T, F> Widget for StackedLayout<H, T, F>
where
    H: Widget,
    T: Widget,
    F: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        // If the area is too small, do nothing.
        if area.height < 2 {
            return;
        }

        let [header_area, content_area, footer_area] = Self::layout(area);

        let content_block = Block::bordered().borders(Borders::TOP | Borders::BOTTOM);
        let inner_content_area = content_block.inner(content_area);
        content_block.render(content_area, buf);

        self.header.render(header_area, buf);
        self.content.render(inner_content_area, buf);
        self.footer.render(footer_area, buf);
    }
}
