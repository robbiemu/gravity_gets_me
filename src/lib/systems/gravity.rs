use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::lib::prelude::{Particle, Player};


const GRAVITY_CONSTANT: f32 = 1.0; // Adjust as necessary for your game
const MIN_DISTANCE: f32 = 10.0; // Minimum distance to avoid extreme forces

pub fn apply_custom_gravity(
  mut query: Query<(&Transform, &Particle, &mut Velocity), Without<Player>>,
  player_query: Query<(&Transform, &Particle), With<Player>>,
) {
  // Get the player's position and mass
  let (player_transform, player_particle) = player_query.single();
  let player_position = player_transform.translation.truncate();
  let player_mass = player_particle.mass;

  // For every non-player particle, apply gravity
  for (transform, particle, mut velocity) in query.iter_mut() {
    let particle_position = transform.translation.truncate();
    let direction = player_position - particle_position;

    let distance = direction.length();
    // Prevent extreme forces for very close particles
    if distance < MIN_DISTANCE {
      continue;
    }
    let distance_squared = direction.length_squared();

    // Calculate the gravitational force magnitude
    let force_magnitude =
      GRAVITY_CONSTANT * (particle.mass * player_mass) / distance_squared;

    // Apply the force to the velocity
    let force = direction.normalize() * force_magnitude;
    velocity.linvel += force;
  }
}
