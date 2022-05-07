use bevy::prelude::*;

use crate::{client::camera::third_person::Anchor, utils::Math};

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
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });

  // cube
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  })
  .insert(Character::default())
  .insert(Anchor::default());
}


fn movement(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,

  mut chars: Query<(&mut Transform, &Character)>,
) {

  if key_input.pressed(KeyCode::W) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = trans.forward().clone();
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }

  if key_input.pressed(KeyCode::S) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = trans.back().clone();
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }

  if key_input.pressed(KeyCode::A) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = trans.left().clone();
      trans.translation += dir * char.speed * time.delta_seconds();
    }
  }

  if key_input.pressed(KeyCode::D) {
    for (mut trans, char) in chars.iter_mut() {
      let dir = trans.right().clone();
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