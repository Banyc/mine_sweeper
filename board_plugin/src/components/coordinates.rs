use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use bevy::prelude::*;

#[derive(Component)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
