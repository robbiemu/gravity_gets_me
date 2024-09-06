use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::particle::Particle;


#[derive(Debug, Component, Clone, PartialEq)]
pub struct Player;


pub fn entity_promote_to_player(entity: Entity, commands: &mut Commands) {
  commands
    .entity(entity)
    .insert(Player)
    .remove::<RigidBody>()
    .insert(RigidBody::Fixed);
}

pub fn swap_player_status(
  current_player: Entity,
  new_player: Entity,
  commands: &mut Commands,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  particles_query: &mut Query<(Entity, &mut Handle<ColorMaterial>, &Particle)>,
  velocities_query: &Query<&Velocity>,
) {
  // Handle the current player if there is one
  if let Ok((_, mut material, particle)) =
    particles_query.get_mut(current_player)
  {
    // Change the current player's color to non-player color
    *material = materials.add(ColorMaterial::from(particle.get_color(false)));
  } else {
    eprintln!("cant find from entity");
  }

  // Calculate average velocity of all particles excluding the current player and the new player
  let mut total_velocity = Vec2::ZERO;
  let particle_count = velocities_query.iter().count();
  for v in velocities_query.iter() {
    total_velocity += v.linvel;
  }
  let average_velocity = total_velocity / (particle_count as f32);

  commands
    .entity(current_player)
    .remove::<Player>()
    .remove::<RigidBody>()
    .insert(RigidBody::Dynamic)
    .remove::<Velocity>()
    .insert(Velocity::linear(average_velocity));

  // Promote the new player
  if let Ok((_, mut material, particle)) = particles_query.get_mut(new_player) {
    // Change the new player's color to player color
    *material = materials.add(ColorMaterial::from(particle.get_color(true)));
  }

  commands
    .entity(new_player)
    .insert(Player)
    .remove::<RigidBody>()
    .insert(RigidBody::Fixed);
}
