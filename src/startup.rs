use bevy::prelude::*;

use crate::lib::components::particle::spawn_particle;
use crate::lib::components::player::entity_promote_to_player;
use crate::lib::prelude::Particle;


pub fn setup(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // Setup camera
  commands.spawn(Camera2dBundle::default());

  // Spawn player's particle with circle shape
  let pp = Particle { mass: 50. };
  let color = pp.get_color(true);
  let player = spawn_particle(pp, &mut commands, &mut materials, color);
  entity_promote_to_player(player, &mut commands);
}
