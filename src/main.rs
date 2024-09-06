use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

mod lib;
use aim::handle_ui_fire_and_aiming;
use game_over::display_game_over_text;
use game_state::AimingDirection;
use game_state::GameMode;
use game_state::GameOverDelayTimer;
use game_state::GameState;
use game_state::GlobalParticleCount;
use game_state::GravitationalConstant;
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
    .insert_resource(GameState::default())
    .insert_resource(GameOverDelayTimer::default()) // Initialize the delay timer
    .insert_resource(GlobalParticleCount::default())
    .insert_resource(GravitationalConstant::default())
    .insert_resource(AimingDirection::default())
    .add_plugins(DefaultPlugins)
    .add_plugins(ShapePlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_systems(Startup, setup)
    .add_systems(Update, display_game_over_text)
    .add_systems(Update, handle_ui_fire_and_aiming)
    .add_systems(Update, remove_off_screen_particles)
    .add_systems(Update, replace_deferred_particles)
    .add_systems(Update, apply_custom_gravity)
    .run();
}
