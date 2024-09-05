use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::*;

use crate::{
  aim,
  game_state::AimingDirection,
  lib::{
    components::player::swap_player_status,
    prelude::{Laser, Particle, Player},
  },
};


#[allow(clippy::too_many_arguments)]
pub fn handle_ui_fire_and_aiming(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
  rapier_context: ResMut<RapierContext>,
  mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
  mut aiming_direction: ResMut<AimingDirection>,
  mut laser_query: Query<(Entity, &mut Path), With<Laser>>,
  mut player_query: Query<(Entity, &Transform), With<Player>>,
  mut particle_query: Query<(Entity, &mut Handle<ColorMaterial>, &Particle)>, // Query for particle entities
) {
  let (player_id, player_transform) = player_query.single_mut();

  // Update angle based on input
  let is_drawing_line = keyboard_input.pressed(KeyCode::Space)
    || keyboard_input.pressed(KeyCode::ArrowLeft)
    || keyboard_input.pressed(KeyCode::ArrowRight);

  // Rotate aiming direction with arrow keys
  let rotation_speed = 0.01;
  let mut rotation = 0.;
  if keyboard_input.pressed(KeyCode::ArrowLeft) {
    rotation = -rotation_speed;
    eprintln!("rotated {rotation}");
  }
  if keyboard_input.pressed(KeyCode::ArrowRight) {
    rotation = rotation_speed;
    eprintln!(
      "rotated {rotation}, {:?} @ {:?} -> {:?} or {:?}",
      aiming_direction.direction,
      Vec2::from_angle(rotation),
      Vec2::from_angle(rotation).rotate(aiming_direction.direction),
      aiming_direction
        .direction
        .rotate(Vec2::from_angle(rotation))
    );
  }
  aiming_direction.direction =
    Vec2::from_angle(rotation).rotate(aiming_direction.direction);


  // Draw line indicating shooting trajectory
  // Draw line indicating shooting trajectory
  if is_drawing_line {
    // Calculate shooting line start and end
    let start = player_transform.translation.truncate();
    let end = start + aiming_direction.direction * 1000.0; // Adjust length as needed

    dbg!(start, end);

    // Check if there's an existing laser
    let (laser, mut path) = laser_query.single_mut();
    *path = GeometryBuilder::build_as(&shapes::Line(start, end));
    commands.entity(laser).remove::<Visibility>();
    commands.entity(laser).insert(Visibility::Visible);
  } else {
    // Hide the line when no button is pressed\
    let (laser, _path) = laser_query.single();
    commands.entity(laser).insert(Visibility::Hidden);
  }

  // Handle result of shooting
  if keyboard_input.just_released(KeyCode::Space) {
    keyboard_input.clear_just_pressed(KeyCode::Space);

    let ray_pos = player_transform.translation.truncate();
    let ray_dir = aiming_direction.direction;
    let max_toi = 1000.0; // Length of the ray
    let solid = true;

    // Exclude player entity from raycast
    let filter = QueryFilter::default().exclude_rigid_body(player_id);

    if let Some((entity, toi)) =
      rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
    {
      let hit_point = ray_pos + ray_dir * toi;
      println!(
        "Entity {:?} hit at point {:?} with ray_pos: {:?} and direction: {:?}",
        entity, hit_point, ray_pos, ray_dir
      );

      // Replace the player with the particle entity (or handle the swap logic)
      swap_player_status(
        Some(player_id),
        entity,
        &mut commands,
        &mut materials,
        &mut particle_query,
      );
    } else {
      println!(
        "No entity hit with ray_pos: {:?} and direction: {:?}",
        ray_pos, ray_dir
      );
    }
  }
}
