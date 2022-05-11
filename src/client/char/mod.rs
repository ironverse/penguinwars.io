use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use voxels::chunk::{world_pos_to_key, voxel_pos_to_key};
use crate::{utils::{Math, to_key}};
use super::{camera::Anchor, GameResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_startup_system(add_ground)
      .add_startup_system(add_char);

    app
      .add_system(movement);
      
  }
}

fn add_ground(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let ground_size = 1.0;
  let ground_height = 0.1;

  commands
    .spawn()
    .insert(Collider::cuboid(ground_size * 0.5, ground_height, ground_size * 0.5))
    .insert(Transform::from_xyz(0.0, -ground_height, 0.0))
    .insert(GlobalTransform::default());
  
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size * 1.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
}

fn add_char(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  res: Res<GameResource>,
) {
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

  let pos = Vec3::new(0.0, 5.0, 0.0);
  let seamless_size = res.chunk_manager.seamless_size();
  let char = Character {
    // cur_key: world_pos_to_key(&[pos.x as i64, pos.y as i64, pos.y as i64], seamless_size),
    cur_key: voxel_pos_to_key(&[pos.x as i64, pos.y as i64, pos.y as i64], seamless_size),
    ..default()
  };
  commands
    .spawn()
    .insert(RigidBody::Dynamic)
    .insert(Collider::capsule_y(depth * 0.5, radius))
    .insert(Transform::from_translation(pos))
    .insert(GlobalTransform::default())
    .insert(char)
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

fn movement(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  res: Res<GameResource>,

  anchors: Query<&Anchor>,
  mut chars: Query<(&mut Transform, &mut Character, &mut ExternalImpulse)>,
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
  

  let force = 50.0;
  let mut inter_force = direction * force * time.delta_seconds();
  for (mut trans, mut char, mut ext_impulse) in chars.iter_mut() {
    let key = to_key(&trans.translation, res.chunk_manager.seamless_size());
    if char.cur_key != key {
      char.prev_key = char.cur_key.clone();
      char.cur_key = key;
    }

    if direction == Vec3::ZERO {
      return;
    }

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


#[derive(Component)]
pub struct Character {
  pub speed: f32,
  pub prev_key: [i64; 3],
  pub cur_key: [i64; 3],
}

impl Default for Character {
  fn default() -> Self {
    Self {
      speed: 5.0,
      prev_key: [i64::MIN, i64::MIN, i64::MIN],
      cur_key: [i64::MIN, i64::MIN, i64::MIN],
    }
  }
}



