use bevy::{math::const_vec3, prelude::*};

#[derive(Debug, Copy, Clone, Component)]
pub struct BagSize {
    width: u8,
    height: u8,
}

impl BagSize {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    pub const fn width(self) -> u8 {
        self.width
    }

    pub const fn height(self) -> u8 {
        self.height
    }

    pub const fn half_width(self) -> f32 {
        self.width as f32 / 2.
    }

    pub const fn half_height(self) -> f32 {
        self.height as f32 / 2.
    }

    pub const fn origin(self) -> Vec3 {
        const_vec3!([self.half_width(), self.half_height(), 0.])
    }

    pub const fn capacity(self) -> u8 {
        self.height * self.width
    }
}
