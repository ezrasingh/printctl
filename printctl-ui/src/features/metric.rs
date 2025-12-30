use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::time::Duration;

#[derive(Debug, Default, Clone, Copy)]
pub enum Units {
    Inches,
    #[default]
    Millimeters,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ActivePlane {
    #[default]
    XY,
    XZ,
    YZ,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum PositionMode {
    #[default]
    Relative,
    Absolute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance {
    mm: f32,
}

impl Distance {
    pub fn new(dist: f32, units: &Units) -> Self {
        match units {
            Units::Inches => Self::from_inches(dist),
            Units::Millimeters => Self::from_mm(dist),
        }
    }
}

impl Distance {
    pub const MM_PER_INCH: f32 = 25.4;
    pub const ZERO: Self = Self { mm: 0.0 };

    #[inline]
    pub fn from_mm(mm: f32) -> Self {
        Self { mm }
    }

    #[inline]
    pub fn from_inches(inches: f32) -> Self {
        Self {
            mm: inches * Self::MM_PER_INCH,
        }
    }

    #[inline]
    pub fn as_mm(self) -> f32 {
        self.mm
    }

    #[inline]
    pub fn as_inches(self) -> f32 {
        self.mm / Self::MM_PER_INCH
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.mm <= f32::EPSILON
    }
}

impl Add for Distance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_mm(self.mm + rhs.mm)
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, rhs: Self) {
        self.mm += rhs.mm;
    }
}

impl Sub for Distance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_mm(self.mm - rhs.mm)
    }
}

impl SubAssign for Distance {
    fn sub_assign(&mut self, rhs: Self) {
        self.mm -= rhs.mm;
    }
}

impl Mul<f32> for Distance {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::from_mm(self.mm * rhs)
    }
}

impl Div<f32> for Distance {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::from_mm(self.mm / rhs)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    x_mm: f32,
    y_mm: f32,
    z_mm: f32,
}

impl Position {
    pub const ORIGIN: Self = Self {
        x_mm: 0.0,
        y_mm: 0.0,
        z_mm: 0.0,
    };

    #[inline]
    pub fn x(&self) -> Distance {
        Distance::from_mm(self.x_mm)
    }

    #[inline]
    pub fn y(&self) -> Distance {
        Distance::from_mm(self.y_mm)
    }

    #[inline]
    pub fn z(&self) -> Distance {
        Distance::from_mm(self.z_mm)
    }

    #[inline]
    pub fn distance(&self, to: &Position) -> Distance {
        let dx = to.x_mm - self.x_mm;
        let dy = to.y_mm - self.y_mm;
        let dz = to.z_mm - self.z_mm;
        let mm = (dx * dx + dy * dy + dz * dz).sqrt();
        Distance::from_mm(mm)
    }

    #[inline]
    pub fn planar_distance(&self, to: &Position, plane: &ActivePlane) -> Distance {
        let mm = match plane {
            ActivePlane::XY => {
                let dx = to.x_mm - self.x_mm;
                let dy = to.y_mm - self.y_mm;
                (dx * dx + dy * dy).sqrt()
            }
            ActivePlane::XZ => {
                let dx = to.x_mm - self.x_mm;
                let dz = to.z_mm - self.z_mm;
                (dx * dx + dz * dz).sqrt()
            }
            ActivePlane::YZ => {
                let dy = to.y_mm - self.y_mm;
                let dz = to.z_mm - self.z_mm;
                (dy * dy + dz * dz).sqrt()
            }
        };

        Distance::from_mm(mm)
    }

    pub fn translate_x(&mut self, dx: Distance, mode: &PositionMode) {
        match mode {
            PositionMode::Absolute => self.x_mm = dx.as_mm(),
            PositionMode::Relative => self.x_mm += dx.as_mm(),
        }
    }

    pub fn translate_y(&mut self, dy: Distance, mode: &PositionMode) {
        match mode {
            PositionMode::Absolute => self.y_mm = dy.as_mm(),
            PositionMode::Relative => self.y_mm += dy.as_mm(),
        }
    }

    pub fn translate_z(&mut self, dz: Distance, mode: &PositionMode) {
        match mode {
            PositionMode::Absolute => self.z_mm = dz.as_mm(),
            PositionMode::Relative => self.z_mm += dz.as_mm(),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x_mm: self.x_mm + rhs.x_mm,
            y_mm: self.y_mm + rhs.y_mm,
            z_mm: self.z_mm + rhs.z_mm,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x_mm += rhs.x_mm;
        self.y_mm += rhs.y_mm;
        self.z_mm += rhs.z_mm;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x_mm: self.x_mm - rhs.x_mm,
            y_mm: self.y_mm - rhs.y_mm,
            z_mm: self.z_mm - rhs.z_mm,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x_mm -= rhs.x_mm;
        self.y_mm -= rhs.y_mm;
        self.z_mm -= rhs.z_mm;
    }
}

impl Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x_mm: self.x_mm * rhs,
            y_mm: self.y_mm * rhs,
            z_mm: self.z_mm * rhs,
        }
    }
}

impl Div<f32> for Position {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x_mm: self.x_mm / rhs,
            y_mm: self.y_mm / rhs,
            z_mm: self.z_mm / rhs,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Speed {
    mm_per_s: f32,
}

impl Speed {
    pub const ZERO: Self = Self { mm_per_s: 0.0 };

    #[inline]
    pub fn from_mm_per_s(v: f32) -> Self {
        Self { mm_per_s: v }
    }

    #[inline]
    pub fn from_mm_per_min(v: f32) -> Self {
        Self { mm_per_s: v / 60.0 }
    }

    #[inline]
    pub fn from_distance_time(d: Distance, t: Duration) -> Self {
        let secs = t.as_secs_f32();
        Self {
            mm_per_s: if secs > 0.0 { d.as_mm() / secs } else { 0.0 },
        }
    }

    #[inline]
    pub fn as_mm_per_s(self) -> f32 {
        self.mm_per_s
    }

    #[inline]
    pub fn as_mm_per_min(self) -> f32 {
        self.mm_per_s * 60.0
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.mm_per_s <= f32::EPSILON
    }
}

impl Div<Speed> for Distance {
    type Output = Duration;

    fn div(self, speed: Speed) -> Duration {
        if speed.mm_per_s <= 0.0 {
            Duration::ZERO
        } else {
            Duration::from_secs_f32(self.mm / speed.mm_per_s)
        }
    }
}

impl Mul<Duration> for Speed {
    type Output = Distance;

    fn mul(self, t: Duration) -> Distance {
        Distance::from_mm(self.mm_per_s * t.as_secs_f32())
    }
}
