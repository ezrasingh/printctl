use std::path::PathBuf;

use gcode::GCode;
use ratatui::widgets::ScrollbarState;

use super::code::{ArgGroups, ArgRange, GCodeLine};

#[derive(Debug)]
pub struct GCodePreview {
    file_path: PathBuf,
    scroll_position: usize,
    scrollbar: ScrollbarState,
    gcode_lines: Box<[GCodeLine]>,
    arg_groups: ArgGroups,
}

impl GCodePreview {
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

impl GCodePreview {
    pub fn file_name(&self) -> &str {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
    }

    pub fn current_line(&self) -> &GCodeLine {
        self.gcode_lines
            .get(self.scroll_position)
            .unwrap_or_default()
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

use super::style::{arg_style, comment_style, gutter_style, opcode_style};

#[inline]
fn render_gcodes<'a>(line: &'a [GCode], is_selected: bool) -> Vec<text::Span<'a>> {
    let mut spans = Vec::new();

    for code in line {
        let mut head = format!("{}{}", code.mnemonic(), code.major_number());

        if code.minor_number() != 0 {
            head.push('.');
            head.push_str(&code.minor_number().to_string());
        }

        if !(matches!(code.major_number(), 0 | 1) && code.arguments().is_empty()) {
            spans.push(text::Span::styled(head, opcode_style(is_selected)));
            spans.push(text::Span::styled(" ", arg_style(is_selected)));
        }

        for arg in code.arguments() {
            spans.push(text::Span::styled(
                arg.letter.to_string(),
                arg_style(is_selected),
            ));

            let value_style = if is_selected {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::LightYellow)
            };

            let value = format!("{}", arg.value);
            let spacer = " ".repeat(9 - value.len());
            spans.push(text::Span::styled(value, value_style));
            spans.push(text::Span::styled(spacer, arg_style(is_selected)));
        }
    }

    spans
}

#[inline]
fn render_comments<'a>(line: &'a [String], in_selected: bool) -> Vec<text::Span<'a>> {
    line.iter()
        .map(|comment| text::Span::styled(comment, comment_style(in_selected)))
        .collect()
}

#[inline]
fn render_gcodes_summary<'a>(line: &'a [GCode]) -> Vec<text::Span<'a>> {
    let mut spans = Vec::new();

    for code in line {
        let mut head = format!("{}{}", code.mnemonic(), code.major_number());

        if code.minor_number() != 0 {
            head.push('.');
            head.push_str(&code.minor_number().to_string());
        }

        if !(matches!(code.major_number(), 0 | 1) && code.arguments().is_empty()) {
            spans.push(text::Span::styled(head, opcode_style(false)));
            spans.push(text::Span::styled(" ", arg_style(false)));
        }

        for arg in code.arguments() {
            spans.push(text::Span::styled(arg.letter.to_string(), arg_style(false)));

            let value_style = Style::default().fg(Color::LightYellow);

            let value = format!("{}", arg.value);
            let spacer = " ".repeat(9 - value.len());
            spans.push(text::Span::styled(value, value_style));
            spans.push(text::Span::styled(spacer, arg_style(false)));
        }
    }

    spans
}

#[inline]
fn gutter<'a>(line_number: usize, selected: bool) -> text::Span<'a> {
    let g = format!("{:>4} │ ", line_number);
    text::Span::styled(g, gutter_style(selected))
}

impl GCodePreview {
    fn render_lines(&self, selection: Option<usize>) -> Vec<text::Line<'_>> {
        self.gcode_lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                let mut out = Vec::new();
                let line_number = i.saturating_add(1);
                let is_selected = self.scroll_position == i;

                match line {
                    GCodeLine::Empty => {
                        out.push(text::Line::from(vec![
                            gutter(line_number, is_selected),
                            text::Span::styled("╌", Style::default().fg(Color::DarkGray)),
                        ]));
                    }

                    GCodeLine::Command { gcodes, comments } => {
                        let mut spans = vec![gutter(line_number, is_selected)];

                        if !gcodes.is_empty() {
                            spans.extend(render_gcodes(gcodes, is_selected));
                        }

                        if !comments.is_empty() {
                            spans.extend(render_comments(comments, is_selected));
                        }

                        out.push(text::Line::from(spans));

                        if is_selected && !gcodes.is_empty() {
                            let mut expanded_spans = Vec::new();
                            expanded_spans.push(text::Span::raw(" ".repeat(7)));
                            expanded_spans.extend(render_gcodes_summary(gcodes));

                            out.push(text::Line::from(expanded_spans));
                        }
                    }
                }

                out
            })
            .collect()
    }

    fn render_arg_groups(&self) -> text::Text<'_> {
        let groups = self.current_arg_group();

        let lines = groups.into_iter().map(|group| {
            let arg = group.argument();
            text::Line::from(vec![
                text::Span::raw(" ".repeat(8)),
                text::Span::styled(
                    format!("{} = {}", arg.letter, arg.value), // e.g. "Z0.500" or "F1800"
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        });

        text::Text::from_iter(lines)
    }
}

impl GCodePreview {
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

impl Widget for &GCodePreview {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::default();

        let inner = block.inner(area);

        let arg_groups = self.current_arg_group();
        let [arg_area, preview_area] = GCodePreview::layout(inner, arg_groups.len());

        Paragraph::new(self.render_arg_groups())
            .block(block)
            .render(arg_area, buf);

        let page_height = preview_area.height.max(1) as usize;
        let total_lines = self.gcode_lines.len();
        let max_scroll = total_lines.saturating_sub(page_height);

        let page_start = (self.scroll_position / page_height) * page_height;
        let scroll = page_start.min(max_scroll);

        let mut scroll_state = self.scrollbar.position(scroll);

        Paragraph::new(self.render_lines(None))
            .scroll((scroll as u16, 0))
            .render(preview_area, buf);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(preview_area, buf, &mut scroll_state);
    }
}
