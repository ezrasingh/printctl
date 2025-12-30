use std::time::Duration;

use super::machine::{HeaterState, MachineState};
use super::snapshot::Transition;

pub trait ThermalModel: Clone {
    /// Temperature at time `t` (seconds since transition start)
    fn temperature(&self, initial: f32, t: Duration) -> f32;

    /// How long until we reach target temperature
    fn settle_time(&self, initial: f32, target: Option<f32>) -> Duration;
}

#[derive(Debug, Clone)]
pub struct LumpedThermalModel {
    pub ambient: f32,
    pub power_w: f32,
    pub loss_coeff: f32,
    pub heat_capacity: f32,
}

impl ThermalModel for LumpedThermalModel {
    fn temperature(&self, initial: f32, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();

        let k = self.loss_coeff / self.heat_capacity;
        let steady = self.ambient + self.power_w / self.loss_coeff;

        steady + (initial - steady) * (-k * t_sec).exp()
    }

    fn settle_time(&self, initial: f32, target: Option<f32>) -> Duration {
        let Some(target) = target else {
            return Duration::ZERO;
        };

        let steady = self.ambient + self.power_w / self.loss_coeff;
        let ratio = (target - steady) / (initial - steady);

        if ratio <= f32::EPSILON {
            return Duration::ZERO;
        }

        let k = self.loss_coeff / self.heat_capacity;
        let t = -ratio.abs().ln() / k;

        Duration::from_secs_f32(t.max(0.0))
    }
}

#[derive(Debug)]
pub struct HeaterTransition<M>
where
    M: ThermalModel,
{
    thermal_model: M,
    heater: HeaterState,
}

impl<M> HeaterTransition<M>
where
    M: ThermalModel,
{
    pub fn new(heater: HeaterState, thermal_model: M) -> Self {
        Self {
            heater,
            thermal_model,
        }
    }
}

impl<M> Transition for HeaterTransition<M>
where
    M: ThermalModel,
{
    type Output = f32;

    fn interpolate(&self, tau: f32) -> f32 {
        let tau = tau.clamp(0.0, 1.0);
        let elapsed = self.duration().mul_f32(tau);

        self.thermal_model
            .temperature(self.heater.current_temp(), elapsed)
    }

    fn duration(&self) -> Duration {
        self.thermal_model
            .settle_time(self.heater.current_temp(), self.heater.target_temp())
    }
}

#[derive(Debug)]
pub struct ThermalSnapshot {
    bed_temp: f32,
    tool_temps: Vec<f32>,
}

#[derive(Debug)]
pub struct ThermalTransition<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    bed: HeaterTransition<B>,
    tools: Vec<HeaterTransition<T>>,
}

impl<B, T> ThermalTransition<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    pub fn new(machine: &MachineState, bed_model: B, tool_model: T) -> Self {
        let bed_heater = machine.bed_heater();
        let bed = HeaterTransition::new(bed_heater.clone(), bed_model);

        let tools = machine
            .tools()
            .iter()
            .map(|tool| {
                let heater = tool.heater_state();
                HeaterTransition::new(heater.clone(), tool_model.clone())
            })
            .collect::<Vec<_>>();

        Self { bed, tools }
    }
}

impl<B, T> Transition for ThermalTransition<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    type Output = ThermalSnapshot;

    fn interpolate(&self, tau: f32) -> ThermalSnapshot {
        ThermalSnapshot {
            bed_temp: self.bed.interpolate(tau),
            tool_temps: self.tools.iter().map(|t| t.interpolate(tau)).collect(),
        }
    }

    fn duration(&self) -> Duration {
        self.tools
            .iter()
            .map(|t| t.duration())
            .chain(std::iter::once(self.bed.duration()))
            .max()
            .unwrap_or(Duration::ZERO)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ThermalTransitionBuilder<B, T> {
    bed_model: B,
    tools_model: T,
}

#[derive(Debug, Default, Clone)]
pub struct NoThermalModel;

impl<T> ThermalTransitionBuilder<NoThermalModel, T> {
    pub fn bed_model<B>(self, model: B) -> ThermalTransitionBuilder<B, T>
    where
        B: ThermalModel,
    {
        ThermalTransitionBuilder {
            bed_model: model,
            tools_model: self.tools_model,
        }
    }
}

impl<B> ThermalTransitionBuilder<B, NoThermalModel> {
    pub fn tools_model<T>(self, model: T) -> ThermalTransitionBuilder<B, T>
    where
        T: ThermalModel,
    {
        ThermalTransitionBuilder {
            bed_model: self.bed_model,
            tools_model: model,
        }
    }
}

impl<B, T> ThermalTransitionBuilder<B, T>
where
    B: ThermalModel,
    T: ThermalModel,
{
    pub fn build(self, state: &MachineState) -> ThermalTransition<B, T> {
        let bed_heater = state.bed_heater();
        let bed = HeaterTransition::new(bed_heater.clone(), self.bed_model);

        let tools = state
            .tools()
            .iter()
            .map(|tool| {
                let heater = tool.heater_state();
                HeaterTransition::new(heater.clone(), self.tools_model.clone())
            })
            .collect::<Vec<_>>();

        ThermalTransition { bed, tools }
    }
}
