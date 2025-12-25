use gcode::GCode;

enum ToolPositioning {
    Relative,
    Absolute,
}

#[derive(Debug, Default)]
pub struct GCodeSimulator {}

use ratatui::style;
use ratatui::widgets::canvas;

impl GCodeSimulator {
    pub fn paint(ctx: &mut canvas::Context, execution: &[GCode]) {
        let mut feed_rate = 1.0;
        let mut positioning: ToolPositioning = ToolPositioning::Relative;
        let mut extruder_positioning: ToolPositioning = ToolPositioning::Relative;
        let mut using_inches = false;
        let mut tool = (0.0, 0.0, 0.0, 0.0);
        let mut extrusion_points: Vec<(f64, f64)> = vec![];
        for code in execution {
            let mut draw_last_position: Option<(f64, f64)> = None;
            match code.mnemonic() {
                gcode::Mnemonic::General => match code.major_number() {
                    0 => match positioning {
                        ToolPositioning::Relative => {
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
                        ToolPositioning::Absolute => {
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
                        ToolPositioning::Relative => {
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
                        ToolPositioning::Absolute => {
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
                        positioning = ToolPositioning::Absolute;
                        extruder_positioning = ToolPositioning::Absolute;
                    }
                    91 => {
                        positioning = ToolPositioning::Relative;
                        extruder_positioning = ToolPositioning::Relative;
                    }
                    _ => {}
                },
                gcode::Mnemonic::Miscellaneous => match code.major_number() {
                    82 => extruder_positioning = ToolPositioning::Absolute,
                    83 => extruder_positioning = ToolPositioning::Relative,
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
