use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{
  game_state::{GlobalParticleCount, GravitationalConstant},
  lib::{
    components::particle::{
      schedule_particle_insertion, spawn_particle, DeferredParticle,
    },
    prelude::{Particle, Player},
  },
};


pub fn replace_deferred_particles(
  mut commands: Commands,
  time: Res<Time>,
  gravitational_constant: Res<GravitationalConstant>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut query: Query<(Entity, &DeferredParticle)>,
) {
  let current_time = time.elapsed_seconds();

  for (entity, deferred_particle) in query.iter_mut() {
    if current_time >= deferred_particle.timestamp {
      // Remove the DeferredParticle component
      commands.entity(entity).remove::<DeferredParticle>();

      // Spawn the actual particle entity
      let p =
        Particle::new_entity_hyperparameters(gravitational_constant.value);
      let entity =
        spawn_particle(p.particle, &mut commands, &mut materials, p.color);
      commands.entity(entity).insert(p.trajectory);
      commands.entity(entity).remove::<Transform>();
      commands
        .entity(entity)
        .insert(Transform::from_translation(p.position));
    }
  }
}

pub fn remove_off_screen_particles(
  mut commands: Commands,
  mut query: Query<
    (Entity, &Transform, &Particle, &mut Velocity),
    Without<Player>,
  >,
  deferred_query: Query<(Entity, &DeferredParticle)>,
  camera_query: Query<&Camera>,
  global_particle_count: Res<GlobalParticleCount>,
  time: Res<Time>,
) {
  // Get the camera's logical viewport size
  let camera = camera_query.single();
  let view_dimensions = camera
    .logical_viewport_size()
    .unwrap_or(Vec2::new(800.0, 600.0)); // Default size if None

  let half_width = view_dimensions.x / 2.0;
  let half_height = view_dimensions.y / 2.0;

  // Use the screen bounds from the camera's viewport
  let mut number_of_particles = deferred_query.iter().count();
  for (entity, transform, _particle, _velocity) in query.iter_mut() {
    let position = transform.translation.truncate();
    if position.x < -half_width
      || position.x > half_width
      || position.y < -half_height
      || position.y > half_height
    {
      // Remove the off-screen particle
      commands.entity(entity).despawn();
    } else {
      number_of_particles += 1;
    }
  }

  // Compare the count with the global value and schedule actions
  if number_of_particles < global_particle_count.value {
    let particles_to_spawn = global_particle_count.value - number_of_particles;

    // Schedule the particles for insertion
    for _ in 0..particles_to_spawn {
      // Example of scheduling a particle; you need to define `schedule_particle_insertion`
      schedule_particle_insertion(&mut commands, &time, 5.0);
    }
  }
}
