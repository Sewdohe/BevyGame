
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

// use crate::entity_util::PhysObjectBundle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;
#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]

pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity: GravityScale,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub damping: Damping,
    pub mass: ColliderMassProperties,
}