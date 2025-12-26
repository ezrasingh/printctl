use std::path::PathBuf;

use gcode::GCode;
use ratatui::widgets::ScrollbarState;

use super::code::{ArgGroups, ArgRange, GCodeLine};

#[derive(Debug)]
pub struct GCodeDebugger {
    file_path: PathBuf,
    scroll_position: usize,
    scrollbar: ScrollbarState,
    gcode_lines: Box<[GCodeLine]>,
    arg_groups: ArgGroups,
}

impl GCodeDebugger {
    pub fn new(path: &PathBuf, src: &str) -> Self {
        let gcode_lines = gcode::full_parse_with_callbacks(src, gcode::Nop)
            .map(|line| GCodeLine::from(line))
            .collect::<Vec<GCodeLine>>()
            .into_boxed_slice();
        let scrollbar = ScrollbarState::default().content_length(gcode_lines.len());
        let arg_groups = ArgGroups::new(&gcode_lines);

        Self {
            scrollbar,
            arg_groups,
            gcode_lines,
            scroll_position: 0,
            file_path: path.to_owned(),
        }
    }
}

impl GCodeDebugger {
    pub fn file_name(&self) -> &str {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
    }

    pub fn current_line(&self) -> &GCodeLine {
        self.gcode_lines
            .get(self.scroll_position)
            .unwrap_or(&GCodeLine::Empty)
    }

    pub fn current_arg_group(&self) -> Vec<&ArgRange> {
        self.arg_groups.get(self.scroll_position)
    }

    pub fn scroll_up(&mut self) {
        self.scroll_position = self.scroll_position.saturating_add(1);
        self.scrollbar = self.scrollbar.position(self.scroll_position);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_position = self.scroll_position.saturating_sub(1);
        self.scrollbar = self.scrollbar.position(self.scroll_position);
    }
}

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text;
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, StatefulWidget, Widget};

use super::style::{arg_style, opcode_style, value_style};

#[inline]
fn gcode_summary_spans<'a>(line: &'a [GCode]) -> Vec<text::Span<'a>> {
    let mut spans = Vec::new();

    for code in line {
        let mut head = format!("{}{}", code.mnemonic(), code.major_number());

        if code.minor_number() != 0 {
            head.push('.');
            head.push_str(&code.minor_number().to_string());
        }

        if !(matches!(code.major_number(), 0 | 1) && code.arguments().is_empty()) {
            spans.extend([
                text::Span::styled(head, opcode_style(false)),
                text::Span::styled(" ", arg_style(false)),
            ])
        }

        for arg in code.arguments() {
            let value = format!("{}", arg.value);
            let spacer = " ".repeat(9 - value.len());
            spans.extend([
                text::Span::styled(arg.letter.to_string(), arg_style(false)),
                text::Span::styled(value, value_style(false)),
                text::Span::styled(spacer, arg_style(false)),
            ])
        }
    }

    spans
}

impl GCodeDebugger {
    fn arg_group_content(&self) -> text::Text<'_> {
        self.current_arg_group()
            .into_iter()
            .map(|range| {
                let arg = range.argument();
                text::Line::from(vec![
                    text::Span::raw(" ".repeat(8)),
                    text::Span::styled(
                        format!("{} = {}", arg.letter, arg.value),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            })
            .collect()
    }

    fn content(&self, selection: Option<usize>) -> Vec<text::Line<'_>> {
        self.gcode_lines
            .iter()
            .enumerate()
            .flat_map(|(i, gcode_line)| {
                let line_number = i + 1;
                let is_selected = self.scroll_position == i;

                let spans = gcode_line.into_spans(line_number, is_selected);
                let mut lines = vec![text::Line::from(spans)];

                if let GCodeLine::Command { gcodes, .. } = gcode_line {
                    if is_selected && !gcodes.is_empty() {
                        let mut expanded_spans = vec![text::Span::raw(" ".repeat(7))];
                        expanded_spans.extend(gcode_summary_spans(gcodes));
                        lines.push(text::Line::from(expanded_spans));
                    }
                }

                lines
            })
            .collect()
    }
}

impl GCodeDebugger {
    fn layout(area: Rect, arg_group_len: usize) -> [Rect; 2] {
        // min heights to avoid collapse
        let min_arg_height = 1;
        let max_arg_height = area.height.saturating_sub(3); // leave room for editor

        // each argument gets 1 row
        let arg_group_height = (arg_group_len as u16).clamp(min_arg_height, max_arg_height);

        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(arg_group_height),
                ratatui::layout::Constraint::Min(1),
            ])
            .split(area);

        [chunks[0], chunks[1]]
    }
}

impl Widget for &GCodeDebugger {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::default();

        let inner = block.inner(area);

        let arg_groups = self.current_arg_group();
        let [arg_area, preview_area] = GCodeDebugger::layout(inner, arg_groups.len());

        Paragraph::new(self.arg_group_content())
            .block(block)
            .render(arg_area, buf);

        let page_height = preview_area.height.max(1) as usize;
        let total_lines = self.gcode_lines.len();
        let max_scroll = total_lines.saturating_sub(page_height);

        let page_start = (self.scroll_position / page_height) * page_height;
        let scroll = page_start.min(max_scroll);

        let mut scroll_state = self.scrollbar.position(scroll);

        Paragraph::new(self.content(None))
            .scroll((scroll as u16, 0))
            .render(preview_area, buf);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(preview_area, buf, &mut scroll_state);
    }
}
