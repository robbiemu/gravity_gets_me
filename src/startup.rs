use bevy::prelude::*;

use crate::lib::components::particle::spawn_particle;
use crate::lib::components::player::entity_promote_to_player;
use crate::lib::prelude::{LaserBundle, Particle};


pub fn setup(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // Setup camera
  commands.spawn(Camera2dBundle::default());

  spawn_player(&mut commands, &mut materials);

  // add HUD
  let laser_id = commands.spawn(LaserBundle::new()).id();
  let mut laser = commands.entity(laser_id);
  laser.remove::<Visibility>();
  laser.insert(Visibility::Hidden);
}

pub fn spawn_player(
  commands: &mut Commands,
  materials: &mut ResMut<Assets<ColorMaterial>>,
) {
  // Spawn player's particle with circle shape
  let pp = Particle { mass: 50. };
  let color = pp.get_color(true);
  let player = spawn_particle(pp, commands, materials, color);
  entity_promote_to_player(player, commands);
}
