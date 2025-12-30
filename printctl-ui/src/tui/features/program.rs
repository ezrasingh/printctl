use gcode::GCode;

use crate::features::code::GCodeLine;
use crate::features::program::{cmds, GCodeProgram};

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text;
use ratatui::widgets::{
    Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
};

use super::style::{arg_style, opcode_style, value_style};

fn gcode_summary_spans<'a>(lines: &'a [GCode]) -> Vec<text::Span<'a>> {
    let mut spans = Vec::new();

    for code in lines {
        let mut head = format!("{}{}", code.mnemonic(), code.major_number());

        if code.minor_number() != 0 {
            head.push('.');
            head.push_str(&code.minor_number().to_string());
        }

        if !(code.arguments().is_empty()
            && matches!(
                code.major_number(),
                cmds::gcode::TRAVEL_MOVE | cmds::gcode::PRINT_MOVE
            ))
        {
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

impl<'a> Into<text::Text<'a>> for &'a GCodeProgram {
    fn into(self) -> text::Text<'a> {
        self.lines()
            .iter()
            .enumerate()
            .flat_map(|(i, gcode_line)| {
                let line_number = i + 1;
                let is_selected = self.cursor() == i;

                let spans = gcode_line.to_spans(line_number, is_selected);
                let mut lines = vec![text::Line::from(spans)];

                if let GCodeLine::Command { gcodes, .. } = gcode_line {
                    if is_selected && !gcodes.is_empty() {
                        let mut expanded_spans = vec![text::Span::raw(" ".repeat(7))];
                        let summary = gcode_summary_spans(gcodes);

                        expanded_spans.extend(summary);
                        lines.push(text::Line::from(expanded_spans));
                    }
                }

                lines
            })
            .collect()
    }
}

impl GCodeProgram {
    fn layout(area: Rect, arg_group_len: usize) -> [Rect; 2] {
        // min heights to avoid collapse
        let min_arg_height = 1;
        let max_arg_height = area.height.saturating_sub(3); // leave room for editor preview

        // each argument gets 1 row
        let arg_group_height = (arg_group_len as u16).clamp(min_arg_height, max_arg_height);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(arg_group_height), Constraint::Min(1)])
            .split(area);

        [chunks[0], chunks[1]]
    }
}

impl StatefulWidget for &GCodeProgram {
    type State = ScrollbarState;

    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        scroll_state: &mut Self::State,
    ) {
        let block = Block::default();
        let inner = block.inner(area);
        let arg_groups = self.current_arg_group();
        let [arg_area, preview_area] = GCodeProgram::layout(inner, arg_groups.len());

        Paragraph::new(
            arg_groups
                .into_iter()
                .map(|arg_range| {
                    let spans = arg_range.to_spans();
                    text::Line::from(spans)
                })
                .collect::<Vec<_>>(),
        )
        .block(block)
        .render(arg_area, buf);

        let total_lines = self.lines().len();
        let page_height = preview_area.height.max(1) as usize;
        let max_scroll = total_lines.saturating_sub(page_height);

        let page_start = (self.cursor() / page_height) * page_height;
        let scroll = page_start.min(max_scroll);

        let mut scrollbar = scroll_state.position(scroll);

        Paragraph::new(self)
            .scroll((scroll as u16, 0))
            .render(preview_area, buf);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(preview_area, buf, &mut scrollbar);
    }
}
