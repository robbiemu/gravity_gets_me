use bevy::prelude::*;

mod lib;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use lib::systems::gravity::apply_custom_gravity;
mod startup;
use startup::setup;


fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ShapePlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_systems(Startup, setup)
    .add_systems(Update, apply_custom_gravity)
    .run();
}
