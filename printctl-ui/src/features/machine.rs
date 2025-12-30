use std::time::Duration;

use gcode::GCode;

use super::metric::{ActivePlane, Distance, Position, PositionMode, Speed, Units};
use super::motion::MotionProfile;

#[derive(Debug, Default, Clone)]
pub struct HomedAxes {
    x: bool,
    y: bool,
    z: bool,
}

#[derive(Debug, Default, Clone)]
pub struct HeaterState(f32, Option<f32>);

impl HeaterState {
    pub fn current_temp(&self) -> f32 {
        self.0
    }

    pub fn target_temp(&self) -> Option<f32> {
        self.1
    }
}

#[derive(Debug, Clone)]
pub struct ToolState(Distance, HeaterState);

impl ToolState {
    pub fn extrusion(&self) -> &Distance {
        &self.0
    }

    pub fn heater_state(&self) -> &HeaterState {
        &self.1
    }

    fn extrude(&mut self, dist: Distance, mode: &PositionMode) {
        match mode {
            PositionMode::Absolute => self.0 = dist,
            PositionMode::Relative => self.0 += dist,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FanState(u8);

impl FanState {
    pub fn speed(&self) -> u8 {
        self.0
    }

    fn stop(&mut self) {
        self.0 = 0;
    }
}

#[derive(Debug, Clone, Default)]
pub struct MachineState {
    units: Units,
    axes: Position,
    feedrate: Speed,
    homed: HomedAxes,

    active_plane: ActivePlane,
    positioning: PositionMode,
    extrusion_positioning: PositionMode,

    active_tool: u8,
    tools: Vec<ToolState>,

    fans: Vec<FanState>,
    cooling_fan: FanState,
    bed_temp: HeaterState,
}

impl MachineState {
    pub fn position(&self) -> Position {
        self.axes
    }

    pub fn plane(&self) -> ActivePlane {
        self.active_plane
    }

    pub fn bed_heater(&self) -> &HeaterState {
        &self.bed_temp
    }

    pub fn tools(&self) -> &Vec<ToolState> {
        &self.tools
    }

    pub fn current_tool(&self) -> &ToolState {
        &self.tools[self.active_tool as usize]
    }

    fn set_feedrate(&mut self, feedrate: Speed) {
        self.feedrate = feedrate;
    }

    fn current_tool_mut(&mut self) -> &mut ToolState {
        &mut self.tools[self.active_tool as usize]
    }
}

use super::program::cmds;

impl MachineState {
    pub fn execute(&self, gcode: &GCode) -> (Self, Option<MotionProfile>) {
        let mut next = self.clone();
        let mut motion = None;

        match (gcode.mnemonic(), gcode.major_number()) {
            (gcode::Mnemonic::General, cmds::gcode::USE_IMPERIAL_UNITS) => {
                next.units = Units::Inches;
            }

            (gcode::Mnemonic::General, cmds::gcode::USE_METRIC_UNITS) => {
                next.units = Units::Millimeters;
            }

            (gcode::Mnemonic::General, cmds::gcode::ABSOLUTE_POSITIONING) => {
                next.positioning = PositionMode::Absolute;
            }

            (gcode::Mnemonic::General, cmds::gcode::RELATIVE_POSITIONING) => {
                next.positioning = PositionMode::Relative;
            }

            (gcode::Mnemonic::Miscellaneous, cmds::mcode::EXTRUDE_ABSOLUTE_POSITIONING) => {
                next.extrusion_positioning = PositionMode::Absolute;
            }

            (gcode::Mnemonic::Miscellaneous, cmds::mcode::EXTRUDE_RELATIVE_POSITIONING) => {
                next.extrusion_positioning = PositionMode::Relative;
            }
            (gcode::Mnemonic::General, cmds::gcode::AUTO_HOME) => {
                next.axes = Position::default();
                next.homed = HomedAxes {
                    x: true,
                    y: true,
                    z: true,
                };
            }
            (gcode::Mnemonic::General, cmds::gcode::TRAVEL_MOVE | cmds::gcode::PRINT_MOVE) => {
                if let Some(e) = gcode.value_for('E') {
                    let dist = Distance::from_mm(e);
                    next.current_tool_mut().extrude(dist, &self.positioning);
                }

                if let Some(f) = gcode.value_for('F') {
                    let dist = Distance::new(f, &self.units);
                    next.feedrate = Speed::from_distance_time(dist, Duration::from_secs(60));
                }

                if let Some(x) = gcode.value_for('X') {
                    let dx = Distance::new(x, &self.units);
                    next.axes.translate_x(dx, &self.positioning);
                }

                if let Some(y) = gcode.value_for('Y') {
                    let dy = Distance::new(y, &self.units);
                    next.axes.translate_y(dy, &self.positioning);
                }

                if let Some(z) = gcode.value_for('Z') {
                    let dz = Distance::new(z, &self.units);
                    next.axes.translate_z(dz, &self.positioning);
                }

                motion.replace(MotionProfile::ConstantVelocity(next.feedrate));
            }
            _ => {}
        }

        (next, motion)
    }
}
