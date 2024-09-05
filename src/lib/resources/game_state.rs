use bevy::prelude::*;


#[derive(Resource)]
pub struct GlobalParticleCount {
  pub value: usize,
}

impl Default for GlobalParticleCount {
  fn default() -> Self {
    Self { value: 10 } // or whatever default value you want
  }
}


#[derive(Resource)]
pub struct AimingDirection {
  pub direction: Vec2,
}

impl Default for AimingDirection {
  fn default() -> Self {
    Self { direction: Vec2::new(0., 1.) } // or whatever default value you want
  }
}
