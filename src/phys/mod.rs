use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use bevy_ecs_ldtk::prelude::*;

pub mod components;
pub mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
      app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
          .add_plugins(RapierDebugRenderPlugin::default())
          .add_systems(Update, systems::spawn_wall_collision);
          // .add_systems(Update, systems::spawn_ground_sensor)
          // .add_systems(Update, systems::ground_detection)
          // .add_systems(Update, systems::update_on_ground)
          // .add_systems(Update, systems::process_jointed_platforms)
  }
}
