use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{utils::Math};

use super::camera::Anchor;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(add_char);

    app
      .add_system(movement2);
      // .add_system(movement);
  }
}

fn add_char(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let ground_size = 5.0;
  let ground_height = 0.1;

  commands
    .spawn()
    .insert(Collider::cuboid(ground_size, ground_height, ground_size))
    .insert(Transform::from_xyz(0.0, -ground_height, 0.0))
    .insert(GlobalTransform::default());
  
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size * 2.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });


  let depth = 1.0;
  let radius = 0.5;
  let total_height = (depth * 0.5) + radius;

  let handle = materials.add(
    StandardMaterial {
      base_color: Color::rgba(0.8, 0.7, 0.6, 0.3).into(),
      alpha_mode: AlphaMode::Blend,
      ..default()
    }
  );
  commands
    .spawn()
    .insert(RigidBody::Dynamic)
    .insert(Collider::capsule_y(depth * 0.5, radius))
    .insert(Transform::from_xyz(0.0, 10.0, 0.0))
    .insert(GlobalTransform::default())
    .insert(Character::default())
    .insert(Anchor::default())
    .insert(ExternalImpulse::default())
    .insert(LockedAxes::ROTATION_LOCKED)
    .with_children(|parent| {
      parent.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
          depth: depth,
          radius: radius,
          ..Default::default()
        })),
        material: handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      });
    });
}

fn movement2(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,

  anchors: Query<&Anchor>,
  mut chars: Query<(&mut Transform, &Character, &mut ExternalImpulse)>,
) {
  let (forward, right) = get_directions(&anchors);
  let mut direction = Vec3::ZERO;

  if key_input.pressed(KeyCode::W) {
    direction = forward;
  }
  if key_input.pressed(KeyCode::S) {
    direction = forward * -1.0;
  }
  if key_input.pressed(KeyCode::A) {
    direction = right * -1.0;
  }
  if key_input.pressed(KeyCode::D) {
    direction = right;
  }
  if direction == Vec3::ZERO {
    return;
  }

  let force = 50.0;
  let mut inter_force = direction * force * time.delta_seconds();
  for (mut trans, char, mut ext_impulse) in chars.iter_mut() {
    ext_impulse.impulse = inter_force;
    ext_impulse.torque_impulse = Vec3::ZERO;
  }
}

fn get_directions(anchors: &Query<&Anchor>) -> (Vec3, Vec3) {
  let mut forward = Vec3::ZERO;
  let mut right = Vec3::ZERO;
  for a in anchors.iter() {
    forward = a.dir.clone();
    forward.y = 0.0; // Disable elevation for now
    forward = forward.normalize();

    right = forward.cross(Vec3::Y);
  }
  (forward, right)
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



