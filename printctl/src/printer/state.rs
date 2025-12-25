use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToolState {
    pub temp: f32,
    pub target: f32,
    pub pwm: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub e: f32,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            temp: 0.0,
            target: 0.0,
            pwm: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            e: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrinterState {
    pub tools: HashMap<usize, ToolState>,
    pub bed: ToolState,
    pub fan_speed: u8,
    pub ready: bool,
    pub last_error: Option<String>,
}

impl Default for PrinterState {
    fn default() -> Self {
        Self {
            tools: HashMap::new(),
            bed: ToolState::default(),
            fan_speed: 0,
            ready: false,
            last_error: None,
        }
    }
}

impl PrinterState {
    pub fn update_from_line(&mut self, line: &str) {
        let raw = line.trim();
        if raw.is_empty() {
            return;
        }

        // Error handling
        if raw.contains("Error") {
            self.last_error = Some(raw.to_string());
            return;
        }

        // OK = printer ready
        if raw.starts_with("ok") {
            self.ready = true;
            return;
        }

        // M105 Temperature Report
        // Example:
        //   T:200.0 /200.0 B:60.0 /60.0
        //   T0:200.0 /200.0 T1:205.0 /210.0 B:60.0 /60.0
        if raw.contains("T:") || raw.contains("T0:") || raw.contains("B:") {
            self.parse_temperature_report(raw);
        }

        // Position report (M114)
        // Example:
        //   X:10.00 Y:20.00 Z:0.50 E:1.23
        if raw.contains("X:") && raw.contains("Y:") && raw.contains("Z:") {
            self.parse_position_report(raw);
        }

        // Fan speed (M106/M107)
        // e.g. "ok FAN speed:128"
        if raw.contains("FAN") && raw.contains("speed") {
            self.parse_fan_report(raw);
        }
    }

    fn parse_temperature_report(&mut self, raw: &str) {
        for part in raw.split_whitespace() {
            // Hotends: T: / T0: / T1:
            if part.starts_with('T') && part.contains(':') && !part.starts_with("T:") {
                // Example: T1:200.0/210.0
                let tool_idx = part[1..part.find(':').unwrap()]
                    .parse::<usize>()
                    .unwrap_or(0);
                let temps = &part[part.find(':').unwrap() + 1..];

                if let Some((cur, tgt)) = parse_temp_pair(temps) {
                    let entry = self.tools.entry(tool_idx).or_default();
                    entry.temp = cur;
                    entry.target = tgt;
                }
            }

            // Single extruder T:
            if part.starts_with("T:") {
                let temps = &part[2..];
                if let Some((cur, tgt)) = parse_temp_pair(temps) {
                    let entry = self.tools.entry(0).or_default();
                    entry.temp = cur;
                    entry.target = tgt;
                }
            }

            // Bed temperature B:
            if part.starts_with("B:") {
                let temps = &part[2..];
                if let Some((cur, tgt)) = parse_temp_pair(temps) {
                    self.bed.temp = cur;
                    self.bed.target = tgt;
                }
            }
        }
    }

    fn parse_position_report(&mut self, raw: &str) {
        // Always update tool 0 (Marlin only reports current workspace, not per-tool)
        let tool = self.tools.entry(0).or_default();

        for part in raw.split_whitespace() {
            if let Some(val) = part.strip_prefix("X:") {
                tool.x = parse_f32(val, tool.x);
            }
            if let Some(val) = part.strip_prefix("Y:") {
                tool.y = parse_f32(val, tool.y);
            }
            if let Some(val) = part.strip_prefix("Z:") {
                tool.z = parse_f32(val, tool.z);
            }
            if let Some(val) = part.strip_prefix("E:") {
                tool.e = parse_f32(val, tool.e);
            }
        }
    }

    fn parse_fan_report(&mut self, raw: &str) {
        // naive example: "FAN speed:128"
        if let Some(idx) = raw.find("speed:") {
            if let Ok(val) = raw[idx + 6..].trim().parse::<u8>() {
                self.fan_speed = val;
            }
        }
    }
}

fn parse_temp_pair(s: &str) -> Option<(f32, f32)> {
    let mut parts = s.split('/');
    let cur = parts.next()?.parse::<f32>().ok()?;
    let tgt = parts.next()?.parse::<f32>().ok()?;
    Some((cur, tgt))
}

fn parse_f32(s: &str, fallback: f32) -> f32 {
    s.parse::<f32>().unwrap_or(fallback)
}
