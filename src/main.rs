use bevy::prelude::*;

mod lib;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use game_state::GlobalParticleCount;
use gravity::apply_custom_gravity;
use lib::resources::*;
use lib::systems::*;
mod startup;
use particle_keeper::{
  remove_off_screen_particles, replace_deferred_particles,
};
use startup::setup;


fn main() {
  App::new()
    .insert_resource(GlobalParticleCount::default())
    .add_plugins(DefaultPlugins)
    .add_plugins(ShapePlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_systems(Startup, setup)
    .add_systems(Update, remove_off_screen_particles)
    .add_systems(Update, replace_deferred_particles)
    .add_systems(Update, apply_custom_gravity)
    .run();
}
