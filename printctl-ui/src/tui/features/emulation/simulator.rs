use std::time::Duration;

use gcode::GCode;

use super::stack::GCodeStack;

#[derive(Debug, Default, Clone, Copy)]
pub enum Units {
    Inches,
    #[default]
    Millimeters,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum PositionMode {
    #[default]
    Relative,
    Absolute,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ActivePlane {
    #[default]
    XY,
    XZ,
    YZ,
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone)]
pub struct HomedAxes {
    x: bool,
    y: bool,
    z: bool,
}

#[derive(Debug, Default, Clone)]
pub struct HeaterState {
    current: f32,
    target: Option<f32>,
}

#[derive(Debug, Default, Clone)]
pub struct ToolState {
    e_mm: f32,
    temperature: HeaterState,
}

#[derive(Debug, Default, Clone)]
pub struct FanState(f32);

#[derive(Debug, Clone, Default)]
pub struct ProgramStatistics {
    print_time: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct MachineState {
    units: Units,

    axes: Position,
    homed: HomedAxes,
    active_plane: ActivePlane,

    active_tool: u8,
    tools: Vec<ToolState>,

    feedrate_mm_min: f32,
    fans: Vec<FanState>,
    cooling_fan: FanState,

    positioning: PositionMode,
    extrusion_positioning: PositionMode,
}

#[derive(Debug, Default)]
pub struct GCodeSimulator(GCodeStack, MachineState);

impl GCodeSimulator {
    pub fn new(src: &str) -> Self {
        Self(GCodeStack::new(src), MachineState::default())
    }

    pub fn stack(&self) -> &GCodeStack {
        &self.0
    }

    pub fn stack_mut(&mut self) -> &mut GCodeStack {
        &mut self.0
    }

    pub fn machine(&self) -> &MachineState {
        &self.1
    }

    pub fn machine_mut(&mut self) -> &mut MachineState {
        &mut self.1
    }
}

use ratatui::style;
use ratatui::widgets::{canvas, Block, BorderType, Widget};

impl GCodeSimulator {
    fn paint(ctx: &mut canvas::Context, execution: &[GCode]) {
        let mut feed_rate = 1.0;
        let mut positioning: PositionMode = PositionMode::Relative;
        let mut extruder_positioning: PositionMode = PositionMode::Relative;
        let mut using_inches = false;
        let mut tool = (0.0, 0.0, 0.0, 0.0);
        let mut extrusion_points: Vec<(f64, f64)> = vec![];
        for code in execution {
            let mut draw_last_position: Option<(f64, f64)> = None;
            match code.mnemonic() {
                gcode::Mnemonic::General => match code.major_number() {
                    0 => match positioning {
                        PositionMode::Relative => {
                            draw_last_position = Some((tool.0.clone(), tool.1.clone()));
                            if let Some(x) = code.value_for('X') {
                                tool.0 += x as f64;
                            }
                            if let Some(y) = code.value_for('Y') {
                                tool.1 += y as f64;
                            }
                            if let Some(z) = code.value_for('Z') {
                                tool.2 += z as f64;
                            }
                        }
                        PositionMode::Absolute => {
                            draw_last_position = Some((tool.0.clone(), tool.1.clone()));
                            if let Some(x) = code.value_for('X') {
                                tool.0 = x as f64;
                            }
                            if let Some(y) = code.value_for('Y') {
                                tool.1 = y as f64;
                            }
                            if let Some(z) = code.value_for('Z') {
                                tool.2 = z as f64;
                            }
                        }
                    },
                    1 => match positioning {
                        PositionMode::Relative => {
                            draw_last_position = Some((tool.0.clone(), tool.1.clone()));
                            if let Some(x) = code.value_for('X') {
                                tool.0 += x as f64;
                            }
                            if let Some(y) = code.value_for('Y') {
                                tool.1 += y as f64;
                            }
                            if let Some(z) = code.value_for('Z') {
                                tool.2 += z as f64;
                            }
                            extrusion_points.push((tool.0, tool.1));
                        }
                        PositionMode::Absolute => {
                            draw_last_position = Some((tool.0.clone(), tool.1.clone()));
                            if let Some(x) = code.value_for('X') {
                                tool.0 = x as f64;
                            }
                            if let Some(y) = code.value_for('Y') {
                                tool.1 = y as f64;
                            }
                            if let Some(z) = code.value_for('Z') {
                                tool.2 = z as f64;
                            }
                            extrusion_points.push((tool.0, tool.1));
                        }
                    },
                    20 => using_inches = true,
                    21 => using_inches = false,
                    28 => {
                        draw_last_position = Some((tool.0.clone(), tool.1.clone()));
                        tool.0 = 0.0;
                        tool.1 = 0.0;
                    }
                    90 => {
                        positioning = PositionMode::Absolute;
                        extruder_positioning = PositionMode::Absolute;
                    }
                    91 => {
                        positioning = PositionMode::Relative;
                        extruder_positioning = PositionMode::Relative;
                    }
                    _ => {}
                },
                gcode::Mnemonic::Miscellaneous => match code.major_number() {
                    82 => extruder_positioning = PositionMode::Absolute,
                    83 => extruder_positioning = PositionMode::Relative,
                    _ => {}
                },
                gcode::Mnemonic::ProgramNumber => {}
                gcode::Mnemonic::ToolChange => {}
            }
            if let Some((prev_x, prev_y)) = draw_last_position {
                ctx.draw(&canvas::Line {
                    x1: prev_x,
                    y1: prev_y,
                    x2: tool.0,
                    y2: tool.1,
                    color: style::Color::White,
                });
            }
        }
        ctx.draw(&canvas::Points {
            color: style::Color::Red,
            coords: &extrusion_points.as_slice(),
        });
    }
}

impl Widget for &GCodeSimulator {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let current_execution = self.stack().current_execution();
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
            .render(area, buf);
    }
}
