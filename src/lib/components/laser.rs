use bevy::{color::palettes::css::RED, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[derive(Component, Default)]
pub struct Laser;

#[derive(Bundle)]
pub struct LaserBundle {
  shape_bundle: ShapeBundle,
  stroke: Stroke,
  fill: Fill,
  laser: Laser,
}

impl LaserBundle {
  pub fn new() -> Self {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);
    path_builder.line_to(Vec2::new(1000.0, 1000.0)); // Default line length
    let path = path_builder.build();

    Self {
      shape_bundle: ShapeBundle { path, ..Default::default() },
      stroke: Stroke::new(RED, 5.0), // Adjust thickness as needed
      fill: Fill::color(RED),
      laser: Laser,
    }
  }
}
