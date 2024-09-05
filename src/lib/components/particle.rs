use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Debug, Clone, Default, PartialEq, Component)]
pub struct Particle {
  pub mass: f32,
}

impl Particle {
  pub fn get_size(&self) -> f32 {
    self.mass.powf(2.).ln()
  }

  pub fn get_color(&self, is_player: bool) -> Color {
    if is_player {
      Color::srgb(0.5, 0.7, 0.8)
    } else {
      Color::srgb(1., 0.4, 0.6)
    }
  }
}

pub fn spawn_particle(
  particle: Particle,
  commands: &mut Commands,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  color: Color,
) -> Entity {
  let size = particle.get_size();

  println!(
    "Spawning particle {:?}, size (radius):x {}",
    particle,
    particle.get_size(),
  );

  // Spawn player's particle with circle shape
  commands
    .spawn((
      ShapeBundle {
        path: GeometryBuilder::build_as(&shapes::Circle {
          radius: size, // Radius of the circle
          ..Default::default()
        }),
        material: materials.add(ColorMaterial::from_color(color)),
        ..Default::default()
      },
      RigidBody::Dynamic,
      Collider::ball(size),
      particle,
    ))
    .id()
}
