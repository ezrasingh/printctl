use std::collections::HashMap;
use std::ops::Range;

use gcode::GCode;

#[derive(Debug, Default, Clone)]
pub enum GCodeLine {
    #[default]
    Empty,
    Command {
        gcodes: Box<[GCode]>,
        comments: Box<[String]>,
    },
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct ArgGroups([Box<[ArgRange]>; 26]);

impl ArgGroups {
    fn bucket_id(c: char) -> usize {
        (c as u8 - b'A') as usize
    }

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
                        let idx = Self::bucket_id(letter);

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
            let idx = Self::bucket_id(letter);
            // push at the end -> still preserves increasing start order
            buckets[idx].push(ArgRange::new(start..end, arg));
        }

        let buckets: [Box<[ArgRange]>; 26] =
            std::array::from_fn(|i| std::mem::take(&mut buckets[i]).into_boxed_slice());

        Self(buckets)
    }
}

impl ArgGroups {
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
            // - Ok(i) means line number exactly matches the start of bucket[i]
            // - Err(i) means line number would be inserted at index i to keep the array sorted
            // in both cases, the candidate range that could contain line number is at index i-1
            let check_idx = match idx {
                Ok(i) | Err(i) => i.checked_sub(1),
            };

            if let Some(i) = check_idx {
                let arg = &bucket[i];
                // only add if line number actually falls within the range
                if arg.range().contains(&line_number) {
                    active_args.push(arg);
                }
            }
        }

        active_args
    }
}

impl From<&[GCodeLine]> for ArgGroups {
    fn from(lines: &[GCodeLine]) -> Self {
        Self::new(lines)
    }
}

impl From<&Box<[GCodeLine]>> for ArgGroups {
    fn from(lines: &Box<[GCodeLine]>) -> Self {
        Self::new(lines)
    }
}
