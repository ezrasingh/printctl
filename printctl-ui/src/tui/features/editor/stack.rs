use gcode::GCode;

#[derive(Debug, Clone)]
pub struct GCodeStack(usize, Box<[GCode]>);

impl GCodeStack {
    pub fn new(src: &str) -> Self {
        let stack = gcode::parse(src).collect();
        Self(0, stack)
    }

    /// Read only cursor
    pub fn cursor(&self) -> usize {
        self.0
    }

    /// Mutable cursor reference
    fn cursor_mut(&mut self) -> &mut usize {
        &mut self.0
    }

    /// Read only stack
    fn stack(&self) -> &[GCode] {
        &self.1
    }
}

impl GCodeStack {
    /// Returns how many GCode instructions are on the stack
    pub fn len(&self) -> usize {
        self.stack().len()
    }

    pub fn current(&self) -> Option<&GCode> {
        self.stack().get(self.0)
    }

    /// Read only stack up to current instruction
    pub fn current_execution(&self) -> &[GCode] {
        &self.stack()[..self.cursor()]
    }

    /// Move cursor forward (wrap to first instruction)
    pub fn advance(&mut self) {
        let stack = self.stack();
        if !stack.is_empty() {
            self.0 = self.0.saturating_add(1) % stack.len();
        }
    }

    /// Move cursor backward (wrap to last instruction)
    pub fn rewind(&mut self) {
        let stack = self.stack();
        if !stack.is_empty() {
            self.0 = self.0.saturating_sub(1) % stack.len();
        }
    }
}

use ratatui::widgets::canvas;
use ratatui::widgets::{Block, BorderType, Widget};

use crate::tui::features::GCodeSimulator;

impl GCodeStack {
    pub fn simulator(&self) -> impl Widget + '_ {
        let current_execution = self.current_execution();
        canvas::Canvas::default()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("GCode Simulator"),
            )
            .x_bounds([0.0, 180.0])
            .y_bounds([0.0, 180.0])
            .marker(ratatui::symbols::Marker::HalfBlock)
            .paint(|ctx| GCodeSimulator::paint(ctx, current_execution))
    }
}
