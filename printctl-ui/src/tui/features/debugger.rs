use std::path::PathBuf;

use ratatui::widgets::ScrollbarState;

use crate::features::program::GCodeProgram;
use crate::features::simulator::GCodeSimulator;

#[derive(Debug)]
pub struct GCodeDebugger {
    file_path: PathBuf,
    program: GCodeProgram,
    simulator: GCodeSimulator,
    scrollbar: ScrollbarState,
}

impl GCodeDebugger {
    pub fn new(path: &PathBuf) -> Self {
        let src = std::fs::read_to_string(path).expect("Could not read GCode file");

        Self {
            file_path: path.to_owned(),
            program: GCodeProgram::new(&src),
            simulator: GCodeSimulator::default(),
            scrollbar: ScrollbarState::default(),
        }
    }
}

impl GCodeDebugger {
    fn file_name(&self) -> &str {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
    }

    fn scroll_up(&mut self) {
        self.program.rewind();
    }

    fn scroll_down(&mut self) {
        self.program.advance();
    }
}

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{StatefulWidget, Widget};

use crate::tui::input::{AppEvent, EventHandler};

impl GCodeDebugger {
    fn layout(area: Rect) -> [Rect; 2] {
        let chunks = if area.width < 90 {
            // Vertical fallback
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
        } else {
            // horizontal layout
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
        };

        [chunks[0], chunks[1]]
    }
}

impl Widget for &GCodeDebugger {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let total_lines = self.program.lines().len();
        let [editor_area, simulator_area] = GCodeDebugger::layout(area);
        let mut scrollbar = self.scrollbar.content_length(total_lines);

        self.program.render(editor_area, buf, &mut scrollbar);
    }
}

use crossterm::event::KeyCode;

impl EventHandler for GCodeDebugger {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) -> Option<AppEvent> {
        match key_event.code {
            KeyCode::Up => self.scroll_up(),
            KeyCode::Down => self.scroll_down(),
            //KeyCode::Left => self.simulator_mut().stack_mut().rewind(),
            //KeyCode::Right => self.simulator_mut().stack_mut().advance(),
            _ => {}
        }

        None
    }
}
