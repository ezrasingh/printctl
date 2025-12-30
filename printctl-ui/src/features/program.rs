use std::ops::Range;

use gcode::GCode;

use super::code::{ArgGroups, ArgRange, GCodeLine};

#[derive(Debug, Default, Clone)]
pub struct GCodeProgram {
    stack: Box<[GCode]>,
    arg_groups: ArgGroups,
    lines: Box<[GCodeLine]>,
    selection: Range<usize>,
}

impl GCodeProgram {
    pub fn new(src: &str) -> Self {
        let stack = gcode::full_parse_with_callbacks(src, gcode::Nop)
            .flat_map(|line| line.gcodes().to_vec())
            .collect();
        let lines = gcode::full_parse_with_callbacks(src, gcode::Nop)
            .map(|line| GCodeLine::from(line))
            .collect::<Vec<GCodeLine>>()
            .into_boxed_slice();
        let arg_groups = ArgGroups::from(&lines);

        Self {
            stack,
            lines,
            arg_groups,
            ..Default::default()
        }
    }
}

impl GCodeProgram {
    pub fn cursor(&self) -> usize {
        self.selection.end
    }

    pub fn stack(&self) -> &[GCode] {
        &self.stack
    }

    pub fn lines(&self) -> &[GCodeLine] {
        &self.lines
    }

    pub fn selection(&self) -> &Range<usize> {
        &self.selection
    }

    pub fn arg_groups(&self) -> &ArgGroups {
        &self.arg_groups
    }

    pub fn current_line(&self) -> &GCodeLine {
        self.lines().get(self.cursor()).unwrap_or(&GCodeLine::Empty)
    }

    pub fn current_arg_group(&self) -> Vec<&ArgRange> {
        self.arg_groups.get(self.cursor())
    }

    pub fn executed(&self) -> &[GCode] {
        &self.stack()[..self.cursor()]
    }

    pub fn remaining(&self) -> &[GCode] {
        &self.stack()[self.cursor()..]
    }

    pub fn selection_mut(&mut self) -> &mut Range<usize> {
        &mut self.selection
    }

    pub fn advance(&mut self) -> Option<usize> {
        if self.selection.end < self.stack.len() {
            self.selection.end += 1;
            Some(self.selection.end)
        } else {
            None
        }
    }

    pub fn rewind(&mut self) -> Option<usize> {
        if self.selection.end > self.selection.start {
            self.selection.end -= 1;
            Some(self.selection.end)
        } else {
            None
        }
    }
}

pub mod cmds {
    pub mod gcode {
        // non-modal modes
        pub const DWELL: u32 = 4;
        pub const SET_COORD_SYS: u32 = 10;
        pub const USE_XY_PLANE: u32 = 17;
        pub const USE_ZX_PLANE: u32 = 18;
        pub const USE_YZ_PLANE: u32 = 19;
        pub const USE_IMPERIAL_UNITS: u32 = 20;
        pub const USE_METRIC_UNITS: u32 = 21;
        pub const AUTO_HOME: u32 = 28;
        pub const SINGLE_Z_PROBE: u32 = 30;
        pub const MOVE_IN_MACHINE_COORD: u32 = 53;
        pub const ABSOLUTE_POSITIONING: u32 = 90;
        pub const RELATIVE_POSITIONING: u32 = 91;
        pub const SET_POSITION: u32 = 92;

        // motion modes
        pub const TRAVEL_MOVE: u32 = 0;
        pub const PRINT_MOVE: u32 = 1;
        pub const PRINT_ARC_CW: u32 = 2;
        pub const PRINT_ARC_CCW: u32 = 3;
        pub const PROBE: u32 = 38;
        pub const CANCEL_MOTION_MODE: u32 = 80;
    }

    pub mod mcode {
        // extrusion
        pub const EXTRUDE_ABSOLUTE_POSITIONING: u32 = 82;
        pub const EXTRUDE_RELATIVE_POSITIONING: u32 = 83;
        pub const SET_HOTEND_TEMP: u32 = 104;

        // fan
        pub const SET_FAN_SPEED: u32 = 106;
        pub const FAN_OFF: u32 = 107;

        // temp
        pub const SET_BED_TEMP: u32 = 140;
        pub const SET_CHAMBER_TEMP: u32 = 141;
        pub const SET_TEMP_UNITS: u32 = 149;
        pub const WAIT_FOR_BED_TEMP: u32 = 190;
        pub const WAIT_FOR_CHAMBER_TEMP: u32 = 191;
        pub const WAIT_FOR_PROBE_TEMP: u32 = 192;
    }
}
