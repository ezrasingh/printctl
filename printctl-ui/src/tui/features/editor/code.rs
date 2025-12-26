use std::collections::HashMap;
use std::ops::Range;

use gcode::GCode;
use ratatui::text;

#[derive(Debug, Default)]
pub enum GCodeLine {
    #[default]
    Empty,
    Command {
        gcodes: Box<[GCode]>,
        comments: Box<[String]>,
    },
}

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

use ratatui::style::{Color, Style};

use super::style::{arg_style, comment_style, gutter_style, opcode_style, value_style};

#[inline]
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

#[inline]
fn comment_spans<'a>(line: &'a [String], in_selected: bool) -> Vec<text::Span<'a>> {
    line.iter()
        .map(|comment| text::Span::styled(comment, comment_style(in_selected)))
        .collect()
}

#[inline]
fn gutter_span<'a>(line_number: usize, selected: bool) -> text::Span<'a> {
    text::Span::styled(format!("{:>4} │ ", line_number), gutter_style(selected))
}

impl GCodeLine {
    pub fn into_spans<'a>(&'a self, line_number: usize, is_selected: bool) -> Vec<text::Span<'a>> {
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

#[derive(Debug)]
pub struct ArgRange(Range<usize>, gcode::Word);

impl ArgRange {
    pub fn new(range: Range<usize>, argument: gcode::Word) -> Self {
        Self(range, argument)
    }

    pub fn range(&self) -> &Range<usize> {
        &self.0
    }

    pub fn argument(&self) -> &gcode::Word {
        &self.1
    }
}

#[derive(Debug, Default)]
pub struct ArgGroups([Box<[ArgRange]>; 26]);

impl ArgGroups {
    #[inline]
    fn index(c: char) -> usize {
        (c as u8 - b'A') as usize
    }

    fn buckets(&self) -> &[Box<[ArgRange]>; 26] {
        &self.0
    }

    /// Returns all argument groups active at the given line number
    pub fn get(&self, line_number: usize) -> Vec<&ArgRange> {
        let mut active_args = Vec::new();

        // iterate over each letter's bucket (A-Z) each bucket is sorted by range start.
        for bucket in self.buckets().iter() {
            if bucket.is_empty() {
                // skip empty buckets
                continue;
            }

            // perform a binary search to find the first range whose start is greater than line number
            // this tells us the position where line number would be inserted to maintain sorted order
            let idx = bucket.binary_search_by(|arg| {
                if arg.range().start > line_number {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });

            // after the search:
            // - Ok(i) means `line_number` exactly matches the start of bucket[i]
            // - Err(i) means `line_number` would be inserted at index i to keep the array sorted
            // in both cases, the candidate range that could contain `line_number` is at index i-1
            let check_idx = match idx {
                Ok(i) | Err(i) => i.checked_sub(1),
            };

            if let Some(i) = check_idx {
                let arg = &bucket[i];
                // only add if `line_number` actually falls within the range
                if arg.range().contains(&line_number) {
                    active_args.push(arg);
                }
            }
        }

        active_args
    }
}

impl ArgGroups {
    pub fn new(lines: &[GCodeLine]) -> Self {
        // tracks currently active argument groups by letter
        let mut active: HashMap<char, (usize, gcode::Word)> = HashMap::new();

        // 26 buckets for A-Z, each bucket will hold ArgRanges for that letter
        // invariant: ranges are pushed in strictly increasing order of `start`
        let mut buckets: [Vec<ArgRange>; 26] = std::array::from_fn(|_| Vec::new());

        for (i, line) in lines.iter().enumerate() {
            if let GCodeLine::Command { gcodes, .. } = line {
                for gcode in gcodes.iter() {
                    for arg in gcode.arguments() {
                        let letter = arg.letter.to_ascii_uppercase();
                        let idx = Self::index(letter);

                        match active.get(&letter) {
                            // case: same argument continues -> no action
                            Some((_, active_arg)) if active_arg == arg => continue,

                            // case: previous argument group ends -> close it and start new
                            Some((start, active_arg)) => {
                                let arg_range = ArgRange::new(*start..i, *active_arg);
                                // push to bucket; start always > previous, so bucket remains sorted
                                buckets[idx].push(arg_range);

                                // open new active group starting at current line
                                active.insert(letter, (i, *arg));
                            }

                            // case: first occurrence -> start new group
                            None => {
                                active.insert(letter, (i, *arg));
                            }
                        }
                    }
                }
            }
        }

        // close any remaining active argument groups
        let end = lines.len();
        for (letter, (start, arg)) in active {
            let idx = Self::index(letter);
            // push at the end -> still preserves increasing start order
            buckets[idx].push(ArgRange::new(start..end, arg));
        }

        let buckets: [Box<[ArgRange]>; 26] =
            std::array::from_fn(|i| std::mem::take(&mut buckets[i]).into_boxed_slice());

        Self(buckets)
    }
}
