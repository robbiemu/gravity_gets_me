use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


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
) {
  // Handle the current player if there is one
  if let Some(current_player) = current_player {
    commands
      .entity(current_player)
      .remove::<Player>()
      .remove::<RigidBody>()
      .insert(RigidBody::Dynamic);
  }

  // Promote the new player
  commands
    .entity(new_player)
    .insert(Player)
    .remove::<RigidBody>()
    .insert(RigidBody::Fixed);
}
