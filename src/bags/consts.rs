use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::bags::bag_size::BagSize;

pub const BAG_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b10,
    filters: 0b10,
};
pub const BAG_LID_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b1000,
    filters: 0b1000,
};
pub const BAG_WALLS_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b100,
    filters: 0b100,
};
pub const BAG_FLOOR_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: 0b10000,
    filters: 0b10000,
};
pub const BAG_BOUNDARY_COLLIDER_GROUP: CollisionGroups = CollisionGroups {
    memberships: BAG_WALLS_COLLIDER_GROUP.memberships | BAG_FLOOR_COLLIDER_GROUP.memberships,
    filters: BAG_WALLS_COLLIDER_GROUP.filters | BAG_FLOOR_COLLIDER_GROUP.filters,
};

pub const BAG_SIZE_LARGE: BagSize = BagSize::new(6, 6);
pub const BAG_SIZE_SMALL: BagSize = BagSize::new(3, 4);

pub const LARGEST_BAG_WIDTH: usize = 6;
pub const LARGEST_BAG_HEIGHT: usize = 6;
pub const LARGEST_BAG_CAPACITY: usize = LARGEST_BAG_WIDTH * LARGEST_BAG_HEIGHT;
pub const BAG_SPACING: usize = 2;

pub const LID_HALFHEIGHT: f32 = 0.49;
pub const LID_OFFSET: f32 = 0.5;
pub const BOUNDARY_HALFWIDTH: f32 = 0.009;

pub const BAG_COLOR: Color = Color::rgb(0xC3 as f32 / 255., 0xA9 as f32 / 255., 0x88 as f32 / 255.);
pub const BAG_OUTLINE_COLOR: Color =
    Color::rgb(0x64 as f32 / 255., 0x56 as f32 / 255., 0x46 as f32 / 255.);
