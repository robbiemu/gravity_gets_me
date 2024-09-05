use bevy::prelude::*;

pub use crate::lib::components::particle::Particle;
pub use crate::lib::components::player::Player;


fn get_viewport_bounds(camera_query: &Query<&Camera>) -> (Vec2, Vec2) {
  let projection = camera_query.single();

  // Extract the viewport size from the projection
  let size = projection
    .logical_viewport_size()
    .unwrap_or(Vec2::new(800.0, 600.0)); // Default values if unavailable

  // Define the viewport bounds (bottom-left, top-right)
  let half_width = size.x / 2.0;
  let half_height = size.y / 2.0;

  let bottom_left = Vec2::new(-half_width, -half_height);
  let top_right = Vec2::new(half_width, half_height);

  (bottom_left, top_right)
}
