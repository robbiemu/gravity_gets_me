use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;


#[derive(Debug, Clone, Default, PartialEq, Component)]
pub struct Particle {
  pub mass: f32,
}

#[derive(Component)]
pub struct DeferredParticle {
  pub timestamp: f32, // Timestamp when the particle should be added
}

pub struct ParticleEntityHyprameters {
  pub particle: Particle,
  pub color: Color,
  pub position: Vec3,
  pub trajectory: Velocity,
}

impl Particle {
  pub fn new_entity_hyperparameters() -> ParticleEntityHyprameters {
    let mut rng = rand::thread_rng();
    let particle = Particle { mass: rng.gen_range(1..=1000) as f32 };
    let color = particle.get_color(false);
    let position = Vec3::new(
      rng.gen_range(-500.0..500.0),
      rng.gen_range(-500.0..500.0),
      0.0, // Z-coordinate (0 for 2D games)
    );
    let trajectory = Velocity::linear(Vec2::new(
      rand::random::<f32>() * 100.0,
      rand::random::<f32>() * 100.0,
    ));

    ParticleEntityHyprameters { particle, color, position, trajectory }
  }

  pub fn get_size(&self) -> f32 {
    self.mass.powf(2.).ln() + 1.
  }

  pub fn get_color(&self, is_player: bool) -> Color {
    if is_player {
      Color::srgb(0.5, 0.7, 0.8)
    } else {
      Color::srgb(1., 0.4, 0.6)
    }
  }
}

pub fn schedule_particle_insertion(
  commands: &mut Commands,
  time: &Res<Time>,
  delay: f32,
) {
  // Define the delay in seconds (e.g., 5 seconds)
  let current_time = time.elapsed_seconds();
  let future_time = current_time + delay;

  // Create a deferred particle with a timestamp
  commands.spawn((DeferredParticle { timestamp: future_time },));
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
