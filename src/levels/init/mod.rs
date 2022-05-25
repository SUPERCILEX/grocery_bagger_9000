use bevy::prelude::*;

use crate::{nominos::PiecePlaced, window_management::DipsWindow};

pub mod level01;
pub mod level02;
pub mod level03;
pub mod level04;
pub mod level05;
pub mod level06;
pub mod level07;
pub mod level08;
pub mod level09;
pub mod level10;
pub mod level11;
pub mod level12;
pub mod level13;
pub mod level14;
pub mod level15;
pub mod level16;
pub mod level17;
pub mod level18;
pub mod level19;
pub mod level20;
pub mod level21;
pub mod level22;

pub const LEVELS: &[fn(Commands, Res<DipsWindow>, EventWriter<PiecePlaced>, Res<AssetServer>)] = &[
    level01::init_level,
    level02::init_level,
    level03::init_level,
    level04::init_level,
    level05::init_level,
    level06::init_level,
    level07::init_level,
    level08::init_level,
    level09::init_level,
    level10::init_level,
    level11::init_level,
    level12::init_level,
    level13::init_level,
    level14::init_level,
    level15::init_level,
    level16::init_level,
    level17::init_level,
    level18::init_level,
    level19::init_level,
    level20::init_level,
    level21::init_level,
    level22::init_level,
];
