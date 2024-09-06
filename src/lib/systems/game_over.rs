use bevy::prelude::*;

use crate::{
  game_state::{
    AimingDirection, GameMode, GameOverDelayTimer, GameState,
    GlobalParticleCount, GravitationalConstant,
  },
  lib::prelude::Particle,
  startup::spawn_player,
};


#[allow(clippy::too_many_arguments)]
pub fn display_game_over_text(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut game_state: ResMut<GameState>,
  mut timer: ResMut<GameOverDelayTimer>,
  mut gravitational_constant: ResMut<GravitationalConstant>,
  mut global_particle_count: ResMut<GlobalParticleCount>,
  mut aiming_direction: ResMut<AimingDirection>,
  query: Query<Entity, With<Text>>,
  mut particles_query: Query<Entity, With<Particle>>,
) {
  // Check if the current game mode is GameOver
  if game_state.mode == GameMode::GameOver {
    // If there's no existing GameOver text, spawn it
    if query.is_empty() {
      commands.spawn(TextBundle {
        text: Text::from_section(
          format!(
            "Gravitational Constant was : {:.2}",
            gravitational_constant.value
          ),
          TextStyle {
            font: asset_server.load("fonts/Acme-Regular.ttf"),
            font_size: 50.0,
            color: Color::srgb(1., 1., 1.),
          },
        ),
        ..Default::default()
      });
    }

    // Update the timer (track elapsed time)
    timer.timer.tick(time.delta());

    if timer.timer.finished() && input.get_just_pressed().count() > 0 {
      game_state.mode = GameMode::InPlay;

      // Reset the timer for future use
      timer.timer.reset();

      for entity in particles_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
      }
      gravitational_constant.reset();
      global_particle_count.reset();
      aiming_direction.reset();
      spawn_player(&mut commands, &mut materials);
    }
  } else {
    // Remove the GameOver text when the mode is not GameOver
    for entity in query.iter() {
      commands.entity(entity).despawn();
    }
  }
}
