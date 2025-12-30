use std::time::Duration;

use super::machine::MachineState;
use super::metric::Position;
use super::motion::{MotionProfile, MotionTransition, MotionTransitionBuilder};
use super::thermal::{ThermalModel, ThermalSnapshot, ThermalTransition, ThermalTransitionBuilder};

pub trait Transition {
    type Output;

    /// Interpolate state at normalized time τ
    /// τ is always normalized: [0.0, 1.0]
    fn interpolate(&self, tau: f32) -> Self::Output;

    /// Physical duration of the transition
    fn duration(&self) -> Duration;
}

#[derive(Debug)]
pub struct Snapshot<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    before: MachineState,
    after: MachineState,

    duration: Duration,
    thermal: ThermalTransition<B, T>,
    motion: Option<MotionTransition>,
}

impl<B, T> Snapshot<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    pub fn new(
        before: MachineState,
        after: MachineState,
        thermal: ThermalTransition<B, T>,
        motion: Option<MotionTransition>,
    ) -> Self {
        let duration = thermal.duration().max(
            motion
                .as_ref()
                .map(|m| m.duration())
                .unwrap_or(Duration::ZERO),
        );
        Self {
            duration,
            before,
            after,
            motion,
            thermal,
        }
    }
}

impl<B, T> Transition for Snapshot<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    type Output = (Position, ThermalSnapshot);

    fn interpolate(&self, tau: f32) -> Self::Output {
        let tau = tau.clamp(0.0, 1.0);
        let snapshot_secs = tau * self.duration.as_secs_f32();

        let position = match &self.motion {
            Some(transition) => {
                let t = snapshot_secs / transition.duration().as_secs_f32();
                transition.interpolate(t.clamp(0.0, 1.0))
            }
            None => self.after.position(),
        };

        let thermal = {
            let t = snapshot_secs / self.thermal.duration().as_secs_f32();
            self.thermal.interpolate(t.clamp(0.0, 1.0))
        };

        (position, thermal)
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

#[derive(Debug, Default, Clone)]
pub struct SnapshotBuilder<B, T> {
    bed_thermal_model: B,
    tools_thermal_model: T,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NoThermalModel;

impl<T> SnapshotBuilder<NoThermalModel, T> {
    pub fn bed_thermal_model<B>(self, model: B) -> SnapshotBuilder<B, T>
    where
        B: ThermalModel,
    {
        SnapshotBuilder {
            bed_thermal_model: model,
            tools_thermal_model: self.tools_thermal_model,
        }
    }
}

impl<B> SnapshotBuilder<B, NoThermalModel> {
    pub fn tools_thermal_model<T>(self, model: T) -> SnapshotBuilder<B, T>
    where
        T: ThermalModel,
    {
        SnapshotBuilder {
            bed_thermal_model: self.bed_thermal_model,
            tools_thermal_model: model,
        }
    }
}

impl<B, T> SnapshotBuilder<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    pub fn build(
        self,
        before: MachineState,
        after: MachineState,
        motion_profile: Option<MotionProfile>,
    ) -> Snapshot<B, T> {
        let thermal = ThermalTransitionBuilder::default()
            .bed_model(self.bed_thermal_model)
            .tools_model(self.tools_thermal_model)
            .build(&after);

        let motion = motion_profile.map(|profile| {
            MotionTransitionBuilder::from(profile)
                .start(&before)
                .end(&after)
                .build()
        });

        Snapshot::new(before, after, thermal, motion)
    }
}
