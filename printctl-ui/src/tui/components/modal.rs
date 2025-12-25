use ratatui::{
    layout::{Margin, Rect},
    prelude::{Buffer, Widget},
    widgets::{Block, BorderType},
};

#[derive(Default)]
pub struct EmptyModal;

#[derive(Default)]
pub struct Modal<T> {
    title: String,
    content: T,
}

impl Modal<EmptyModal> {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            content: EmptyModal,
        }
    }

    pub fn content<W>(self, widget: W) -> Modal<W>
    where
        W: Widget,
    {
        Modal {
            title: self.title,
            content: widget,
        }
    }
}

impl<T> Modal<T> {
    pub fn title(self, title: &str) -> Self {
        Modal {
            title: title.into(),
            content: self.content,
        }
    }

    fn layout(area: Rect) -> [Rect; 2] {
        let width = area.width / 2;
        let height = area.height / 3;

        let x = area.x + (area.width - width) / 2;
        let y = area.y + (area.height - height) / 2;

        let modal_area = Rect {
            x,
            y,
            width,
            height,
        };
        let content_area = modal_area.inner(Margin::new(1, 1));

        [modal_area, content_area]
    }
}

impl<T> Widget for Modal<T>
where
    T: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [modal_area, content_area] = Self::layout(area);

        Block::bordered()
            .title(self.title)
            .border_type(BorderType::Rounded)
            .title_alignment(ratatui::layout::Alignment::Center)
            .render(modal_area, buf);

        self.content.render(content_area, buf);
    }
}
