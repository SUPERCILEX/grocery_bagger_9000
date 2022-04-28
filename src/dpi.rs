use std::ops::{Add, Div, Mul, Sub};

use bevy::prelude::*;

use crate::window_utils::PIXELS_PER_UNIT;

#[derive(Debug, Copy, Clone, Deref, DerefMut)]
pub struct Pixels(pub f32);

impl Pixels {
    pub fn dpi(self) -> Dips {
        Dips(self.0 / PIXELS_PER_UNIT)
    }
}

impl Add for Pixels {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pixels(self.0 + rhs.0)
    }
}

impl Add<Dips> for Pixels {
    type Output = Dips;

    fn add(self, rhs: Dips) -> Self::Output {
        self.dpi() + rhs
    }
}

impl Sub for Pixels {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pixels(self.0 - rhs.0)
    }
}

impl Sub<Dips> for Pixels {
    type Output = Dips;

    fn sub(self, rhs: Dips) -> Self::Output {
        self.dpi() - rhs
    }
}

impl Div for Pixels {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Pixels(self.0 / rhs.0)
    }
}

impl Div<Dips> for Pixels {
    type Output = Dips;

    fn div(self, rhs: Dips) -> Self::Output {
        self.dpi() / rhs
    }
}

impl Mul for Pixels {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Pixels(self.0 * rhs.0)
    }
}

impl Mul<Dips> for Pixels {
    type Output = Dips;

    fn mul(self, rhs: Dips) -> Self::Output {
        self.dpi() * rhs
    }
}

#[derive(Debug, Copy, Clone, Deref, DerefMut)]
pub struct Dips(pub f32);

impl Dips {
    pub fn pixels(self) -> Pixels {
        Pixels(self.0 * PIXELS_PER_UNIT)
    }
}

impl Add for Dips {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Dips(self.0 + rhs.0)
    }
}

impl Add<Pixels> for Dips {
    type Output = Pixels;

    fn add(self, rhs: Pixels) -> Self::Output {
        self.pixels() + rhs
    }
}

impl Sub for Dips {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Dips(self.0 - rhs.0)
    }
}

impl Sub<Pixels> for Dips {
    type Output = Pixels;

    fn sub(self, rhs: Pixels) -> Self::Output {
        self.pixels() - rhs
    }
}

impl Div for Dips {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Dips(self.0 / rhs.0)
    }
}

impl Div<Pixels> for Dips {
    type Output = Pixels;

    fn div(self, rhs: Pixels) -> Self::Output {
        self.pixels() / rhs
    }
}

impl Mul for Dips {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Dips(self.0 * rhs.0)
    }
}

impl Mul<Pixels> for Dips {
    type Output = Pixels;

    fn mul(self, rhs: Pixels) -> Self::Output {
        self.pixels() * rhs
    }
}
