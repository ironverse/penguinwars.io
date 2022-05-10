use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{utils::Math};

use super::camera::Anchor;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(add_char)
      .add_startup_system(add_collider);

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
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
  //   material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
  //   ..default()
  // });

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
    // .insert(Collider::cuboid(depth * 0.5, depth * 0.5, depth * 0.5))
    .insert(Transform::from_xyz(0.0, 10.0, 0.0))
    .insert(GlobalTransform::default())
    // .spawn_bundle(PbrBundle {
    //   mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
    //   material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    //   transform: Transform::from_xyz(0.0, 0.0, 1.0),
    //   ..default()
    // })

    .insert(Character::default())
    .with_children(|parent| {
      parent.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
          depth: depth,
          radius: radius,
          ..Default::default()
        })),
        material: handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),

        // mesh: meshes.add(Mesh::from(shape::Cube { size: depth })),
        // material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      });
      // .insert(Character::default())
      // .insert(Anchor::default());
    });
  
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Capsule {
  //     depth: depth,
  //     radius: radius,
  //     ..Default::default()
  //   })),
  //   material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
  //   transform: Transform::from_xyz(0.0, total_height, 0.0),
  //   ..default()
  // })
  // .insert(Character::default())
  // .insert(Anchor::default());


  
}

fn add_collider(
  mut commands: Commands
) {
  

}

fn movement2(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,

  anchors: Query<&Anchor>,
  mut chars: Query<(&mut Transform, &Character)>,
) {
  let mut forward = Vec3::ZERO;
  let mut right = Vec3::ZERO;
  for (mut trans, char) in chars.iter_mut() {
    // info!("trans {:?}", trans);
    // trans.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
  }
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