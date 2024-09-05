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
