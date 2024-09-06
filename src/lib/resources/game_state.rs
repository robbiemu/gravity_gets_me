use bevy::prelude::*;


#[derive(Default, PartialEq)]
pub enum GameMode {
  #[default]
  InPlay,
  GameOver,
}

#[derive(Resource, Default)]
pub struct GameState {
  pub mode: GameMode,
}

#[derive(Resource)]
pub struct GlobalParticleCount {
  pub value: usize,
}

const GLOBAL_PARTICLE_COUNT: usize = 10;
impl GlobalParticleCount {
  pub fn reset(&mut self) {
    self.value = GLOBAL_PARTICLE_COUNT;
  }
}

impl Default for GlobalParticleCount {
  fn default() -> Self {
    Self { value: GLOBAL_PARTICLE_COUNT }
  }
}

const GRAVITATIONAL_CONSTANT: f32 = 0.1;
#[derive(Resource)]
pub struct GravitationalConstant {
  pub value: f32,
}

impl GravitationalConstant {
  pub fn reset(&mut self) {
    self.value = GRAVITATIONAL_CONSTANT;
  }
}

impl Default for GravitationalConstant {
  fn default() -> Self {
    Self { value: GRAVITATIONAL_CONSTANT }
  }
}

#[derive(Resource)]
pub struct AimingDirection {
  pub direction: Vec2,
}

impl AimingDirection {
  pub fn reset(&mut self) {
    self.direction = Vec2::new(0., 1.);
  }
}

impl Default for AimingDirection {
  fn default() -> Self {
    Self { direction: Vec2::new(0., 1.) } // or whatever default value you want
  }
}

#[derive(Resource)]
pub struct GameOverDelayTimer {
  pub timer: Timer,
}

impl Default for GameOverDelayTimer {
  fn default() -> Self {
    Self { timer: Timer::from_seconds(2.5, TimerMode::Once) }
  }
}
