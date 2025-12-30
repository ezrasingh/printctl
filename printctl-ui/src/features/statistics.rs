use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct MotionMetrics {
    pub distance_mm: f64,
    pub time: Duration,

    pub accelerating_distance_mm: f64,
    pub accelerating_time: Duration,

    pub cruising_distance_mm: f64,
    pub cruising_time: Duration,

    pub decelerating_distance_mm: f64,
    pub decelerating_time: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct MotionStatistics {
    pub print: MotionMetrics,
    pub travel: MotionMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct ExtrusionMetrics {
    pub extruded_mm: f64,
    pub retracted_mm: f64,
    pub primed_mm: f64,

    pub extrusion_time: Duration,
    pub retract_time: Duration,
    pub prime_time: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct ProgramStatistics {
    // meta
    pub number_of_lines: usize,
    pub print_moves: usize,
    pub travel_moves: usize,

    // motion
    pub xy_motion: MotionStatistics,
    pub z_motion: MotionMetrics,

    // extrusion
    pub extrusion: ExtrusionMetrics,

    // timing
    pub total_time: Duration,
}
