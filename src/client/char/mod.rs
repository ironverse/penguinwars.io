use bevy::prelude::*;

use crate::{utils::Math};

use super::camera::Anchor;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(add_char);

    app
      .add_system(movement);
  }
}

fn add_char(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
  //   material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
  //   ..default()
  // });

  let depth = 1.0;
  let radius = 0.5;
  let total_height = (depth * 0.5) + radius;
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Capsule {
      depth: depth,
      radius: radius,
      ..Default::default()
    })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, total_height, 0.0),
    ..default()
  })
  .insert(Character::default())
  .insert(Anchor::default());
}


fn movement(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,

  anchors: Query<&Anchor>,
  mut chars: Query<(&mut Transform, &Character)>,
) {
  let mut forward = Vec3::ZERO;
  let mut right = Vec3::ZERO;
  for a in anchors.iter() {
    forward = a.dir.clone();
    forward.y = 0.0; // Disable elevation for now
    forward = forward.normalize();

    right = forward.cross(Vec3::Y);
  }

  if key_input.pressed(KeyCode::W) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = forward;
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }

  if key_input.pressed(KeyCode::S) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = forward * -1.0;
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }

  if key_input.pressed(KeyCode::A) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = right * -1.0;
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }

  if key_input.pressed(KeyCode::D) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = right;
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }
}

#[derive(Component)]
pub struct Character {
  pub speed: f32,
}

impl Default for Character {
  fn default() -> Self {
    Self {
      speed: 5.0
    }
  }
}