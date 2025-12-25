use std::path::PathBuf;

use crate::tui::input::{AppEvent, EventHandler};

use super::preview::GCodePreview;
use super::stack::GCodeStack;

#[derive(Debug)]
pub struct GCodeScript(GCodeStack, GCodePreview);

impl GCodeScript {
    pub fn new(path: &PathBuf) -> Self {
        let src = std::fs::read_to_string(path).expect("Could not read GCode file");
        Self(GCodeStack::new(&src), GCodePreview::new(&path, &src))
    }
}

impl GCodeScript {
    fn stack(&self) -> &GCodeStack {
        &self.0
    }

    fn stack_mut(&mut self) -> &mut GCodeStack {
        &mut self.0
    }

    fn editor(&self) -> &GCodePreview {
        &self.1
    }

    fn editor_mut(&mut self) -> &mut GCodePreview {
        &mut self.1
    }
}

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Paragraph, Widget};

impl GCodeScript {
    fn layout(area: Rect) -> [Rect; 2] {
        if area.width < 90 {
            // Vertical fallback
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            [chunks[0], chunks[1]]
        } else {
            // 3-column horizontal layout
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            [chunks[0], chunks[1]]
        }
    }
}

impl Widget for &GCodeScript {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let stack = self.stack();
        let editor = self.editor();
        let simulator = stack.simulator();
        let [editor_area, simulator_area] = GCodeScript::layout(area);

        editor.render(editor_area, buf);
        simulator.render(simulator_area, buf);
    }
}

use crossterm::event::KeyCode;

impl EventHandler for GCodeScript {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) -> Option<AppEvent> {
        match key_event.code {
            KeyCode::Up => self.editor_mut().scroll_down(),
            KeyCode::Down => self.editor_mut().scroll_up(),
            KeyCode::Left => self.stack_mut().rewind(),
            KeyCode::Right => self.stack_mut().advance(),
            _ => {}
        }

        None
    }
}
