use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::lib::components::particle::spawn_particle;
use crate::lib::components::player::entity_promote_to_player;
use crate::lib::prelude::{Particle, Player};


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

  // Spawn other particles with circle shape
  let mut rng = rand::thread_rng();
  for _ in 0..10 {
    let p = Particle { mass: rng.gen_range(1..=1000) as f32 };
    let color = p.get_color(false);
    let position = Vec3::new(
      rng.gen_range(-500.0..500.0),
      rng.gen_range(-500.0..500.0),
      0.0, // Z-coordinate (0 for 2D games)
    );

    let entity = spawn_particle(p, &mut commands, &mut materials, color);

    let trajectory = Velocity::linear(Vec2::new(
      rand::random::<f32>() * 100.0,
      rand::random::<f32>() * 100.0,
    ));
    commands.entity(entity).insert(trajectory);
    commands.entity(entity).remove::<Transform>();
    commands
      .entity(entity)
      .insert(Transform::from_translation(position));
  }
}
