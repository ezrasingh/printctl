use std::ops::Range;
use std::time::Duration;

use super::machine::MachineState;
use super::program::GCodeProgram;
use super::snapshot::{Snapshot, SnapshotBuilder, Transition};
use super::thermal::ThermalModel;

#[derive(Debug)]
pub struct SnapshotEntry<B, T>(Range<Duration>, Snapshot<B, T>)
where
    B: ThermalModel,
    T: ThermalModel;

impl<B, T> SnapshotEntry<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    pub fn new(start_time: Duration, snapshot: Snapshot<B, T>) -> Self {
        let end_time = start_time + snapshot.duration();
        Self(start_time..end_time, snapshot)
    }

    pub fn time_range(&self) -> &Range<Duration> {
        &self.0
    }

    pub fn start_time(&self) -> Duration {
        self.0.start
    }

    pub fn end_time(&self) -> Duration {
        self.0.end
    }

    pub fn duration(&self) -> Duration {
        self.0.end - self.0.start
    }
}

#[derive(Debug, Default)]
pub struct GCodeSimulator;

impl GCodeSimulator {
    pub fn simulate<B, T>(
        &self,
        program: &GCodeProgram,
        snapshot_builder: SnapshotBuilder<B, T>,
    ) -> (Duration, Vec<SnapshotEntry<B, T>>)
    where
        B: ThermalModel,
        T: ThermalModel,
    {
        let mut time_elapsed = Duration::ZERO;
        let mut state = MachineState::default();
        let mut snapshots = Vec::new();

        for gcode in program.stack() {
            let snapshot = snapshot_builder.clone();
            let (next, motion) = state.execute(gcode);
            let snapshot = snapshot.build(state.clone(), next.clone(), motion);
            let entry = SnapshotEntry::new(time_elapsed, snapshot);

            time_elapsed = entry.end_time();
            snapshots.push(entry);
            state = next;
        }

        (time_elapsed, snapshots)
    }
}
