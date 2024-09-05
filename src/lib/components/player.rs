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
  current_player: Option<Entity>,
  new_player: Entity,
  commands: &mut Commands,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  query: &mut Query<(Entity, &mut Handle<ColorMaterial>, &Particle)>,
) {
  // Handle the current player if there is one
  if let Some(current_player) = current_player {
    if let Ok((_, mut material, particle)) = query.get_mut(current_player) {
      // Change the current player's color to non-player color
      *material = materials.add(ColorMaterial::from(particle.get_color(false)));
    }

    commands
      .entity(current_player)
      .remove::<Player>()
      .remove::<RigidBody>()
      .insert(RigidBody::Dynamic);
  }

  // Promote the new player
  if let Ok((_, mut material, particle)) = query.get_mut(new_player) {
    // Change the new player's color to player color
    *material = materials.add(ColorMaterial::from(particle.get_color(true)));
  }

  commands
    .entity(new_player)
    .insert(Player)
    .remove::<RigidBody>()
    .insert(RigidBody::Fixed);
}
