use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
  game_state::{GameMode, GameState, GravitationalConstant},
  lib::prelude::{Particle, Player},
};


pub fn apply_custom_gravity(
  mut game_state: ResMut<GameState>,
  gravitational_constant: ResMut<GravitationalConstant>,
  mut query: Query<(&Transform, &Particle, &mut Velocity), Without<Player>>,
  player_query: Query<(&Transform, &Particle), With<Player>>,
) {
  // Get the player's position and mass
  let (player_transform, player_particle) = player_query.single();
  let player_position = player_transform.translation.truncate();
  let player_mass = player_particle.mass;
  let player_size = player_particle.get_size();

  // For every non-player particle, apply gravity
  for (transform, particle, mut velocity) in query.iter_mut() {
    let particle_position = transform.translation.truncate();
    let direction = player_position - particle_position;

    let distance = direction.length();
    let min_distance = particle.get_size() + player_size;
    // Prevent extreme forces for very close particles
    if distance <= min_distance {
      game_state.mode = GameMode::GameOver;
      return;
    }
    let distance_squared = direction.length_squared();

    // Calculate the gravitational force magnitude
    let force_magnitude = gravitational_constant.value
      * (particle.mass * player_mass)
      / distance_squared;

    // Apply the force to the velocity
    let force = direction.normalize() * force_magnitude;
    velocity.linvel += force;
  }
}
