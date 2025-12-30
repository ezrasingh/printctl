use gcode::GCode;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text;

use crate::features::code::{ArgRange, GCodeLine};

use super::style::{arg_style, comment_style, gutter_style, opcode_style, value_style};

impl From<gcode::Line<'_>> for GCodeLine {
    fn from(line: gcode::Line<'_>) -> Self {
        let gcodes: Box<[GCode]> = line.gcodes().into();
        let comments: Box<[String]> = line
            .comments()
            .iter()
            .map(|c| c.value.to_string())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        if gcodes.is_empty() && comments.is_empty() {
            GCodeLine::Empty
        } else {
            GCodeLine::Command { gcodes, comments }
        }
    }
}

fn gcode_spans<'a>(line: &'a [GCode], is_selected: bool) -> Vec<text::Span<'a>> {
    let mut spans = Vec::new();

    for code in line {
        let mut head = format!("{}{}", code.mnemonic(), code.major_number());

        if code.minor_number() != 0 {
            head.push('.');
            head.push_str(&code.minor_number().to_string());
        }

        if !(matches!(code.major_number(), 0 | 1) && code.arguments().is_empty()) {
            spans.extend([
                text::Span::styled(head, opcode_style(is_selected)),
                text::Span::styled(" ", arg_style(is_selected)),
            ]);
        }

        for arg in code.arguments() {
            let value = format!("{}", arg.value);
            let spacer = " ".repeat(9 - value.len());
            spans.extend([
                text::Span::styled(arg.letter.to_string(), arg_style(is_selected)),
                text::Span::styled(value, value_style(is_selected)),
                text::Span::styled(spacer, arg_style(is_selected)),
            ]);
        }
    }

    spans
}

fn comment_spans<'a>(line: &'a [String], in_selected: bool) -> Vec<text::Span<'a>> {
    line.iter()
        .map(|comment| text::Span::styled(comment, comment_style(in_selected)))
        .collect()
}

fn gutter_span<'a>(line_number: usize, selected: bool) -> text::Span<'a> {
    text::Span::styled(format!("{:>4} │ ", line_number), gutter_style(selected))
}

impl GCodeLine {
    pub fn to_spans<'a>(&'a self, line_number: usize, is_selected: bool) -> Vec<text::Span<'a>> {
        let mut spans = vec![gutter_span(line_number, is_selected)];

        match self {
            GCodeLine::Empty => spans.push(text::Span::styled(
                "╌",
                Style::default().fg(Color::DarkGray),
            )),

            GCodeLine::Command { gcodes, comments } => {
                if !gcodes.is_empty() {
                    spans.extend(gcode_spans(gcodes, is_selected));
                }

                if !comments.is_empty() {
                    spans.extend(comment_spans(comments, is_selected));
                }
            }
        }

        spans
    }
}

impl ArgRange {
    pub fn to_spans<'a>(&'a self) -> Vec<text::Span<'a>> {
        let arg = self.argument();
        vec![
            text::Span::raw(" ".repeat(8)),
            text::Span::styled(
                format!("{} = {}", arg.letter, arg.value),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    }
}
