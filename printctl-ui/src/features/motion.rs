use std::time::Duration;

use super::machine::MachineState;
use super::metric::{ActivePlane, Distance, Position, Speed};
use super::snapshot::Transition;

#[derive(Debug, Default)]
pub enum MotionProfile {
    ConstantVelocity(Speed),
    #[default]
    Instant, // G92, homing completion, etc
}

#[derive(Debug)]
pub struct MotionTransition {
    start: Position,
    end: Position,
    plane: ActivePlane,
    motion: MotionProfile,
}

impl MotionTransition {
    pub fn new(start: Position, end: Position, plane: ActivePlane, motion: MotionProfile) -> Self {
        Self {
            start,
            end,
            plane,
            motion,
        }
    }

    pub fn distance(&self) -> Distance {
        self.start.planar_distance(&self.end, &self.plane)
    }
}

impl Transition for MotionTransition {
    type Output = Position;

    fn interpolate(&self, tau: f32) -> Position {
        let t = tau.clamp(0.0, 1.0);

        match self.motion {
            MotionProfile::Instant => self.end,

            MotionProfile::ConstantVelocity(_) => {
                let delta = self.end - self.start;
                self.start + delta * t
            }
        }
    }

    fn duration(&self) -> Duration {
        match self.motion {
            MotionProfile::Instant => Duration::ZERO,
            MotionProfile::ConstantVelocity(speed) => self.distance() / speed,
        }
    }
}

#[derive(Debug, Default)]
pub struct MotionTransitionBuilder {
    start: Position,
    end: Position,
    plane: ActivePlane,
    motion: MotionProfile,
}

impl From<MotionProfile> for MotionTransitionBuilder {
    fn from(motion: MotionProfile) -> Self {
        Self {
            motion,
            ..Default::default()
        }
    }
}

impl MotionTransitionBuilder {
    pub fn start(self, state: &MachineState) -> Self {
        Self {
            start: state.position(),
            end: self.end,
            plane: self.plane,
            motion: self.motion,
        }
    }

    pub fn end(self, state: &MachineState) -> Self {
        Self {
            start: self.start,
            end: state.position(),
            plane: state.plane(),
            motion: self.motion,
        }
    }

    pub fn build(self) -> MotionTransition {
        MotionTransition {
            start: self.start,
            end: self.end,
            plane: self.plane,
            motion: self.motion,
        }
    }
}
