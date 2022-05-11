use bevy::prelude::*;
use bevy_rapier3d::{rapier::{prelude::{ColliderFlags, ColliderShape, ColliderPosition, ActiveCollisionTypes}, math::Point}, prelude::Collider};
use voxels::{chunk::{adjacent_keys_by_dist, chunk_manager::{ChunkMode, Chunk}, adjacent_keys, delta_keys, adj_delta_keys, world_pos_to_key, in_range}, data::{voxel_octree::{VoxelMode, VoxelOctree}, surface_nets::{get_surface_nets2}}};

use crate::utils::to_key;

use super::{GameResource, utils::create_mesh, char::Character};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default());
    app
      .add_system(entered_world_keys)
      .add_system(movement_delta_keys)
      .add_system(load_data)
      .add_system(add_meshes)
      .add_system(add_colliders)
      .add_system(remove_colliders);
  }
}

fn entered_world_keys(
  res: Res<GameResource>,
  mut local_res: ResMut<LocalResource>,

  chars: Query<&Character, Added<Character>>,
) {
  for char in chars.iter() {
    let dist0 = res.chunk_manager.lod_dist0;
    let keys0 = adjacent_keys_by_dist(&char.cur_key, dist0);

    local_res.load_keys.extend(keys0.iter());
    local_res.load_mesh_keys.extend(keys0.iter());

    // let col_keys = adjacent_keys_by_dist(&char.key, 1);
    let col_keys = adjacent_keys(&char.cur_key, 1);
    local_res.load_collider_keys.extend(col_keys.iter());
  }
}

fn movement_delta_keys(
  res: Res<GameResource>,
  mut local_res: ResMut<LocalResource>,

  chars: Query<&Character, Changed<Character>>,
) {
  for char in chars.iter() {
    if char.prev_key[0] == i64::MIN {
      continue;
    }

    let range = res.chunk_manager.lod_dist0;
    let keys0 = delta_keys(&char.prev_key, &char.cur_key, range);

    local_res.load_keys.extend(keys0.iter());
    local_res.load_mesh_keys.extend(keys0.iter());

    let col_keys = adj_delta_keys(&char.prev_key, &char.cur_key, 1);
    local_res.load_collider_keys.extend(col_keys.iter());
  }
}


fn load_data(
  mut local_res: ResMut<LocalResource>,
  mut res: ResMut<GameResource>,
) {
  /* TODO: Limit loading data based on time spent, to not lock the whole system */
  let lod = res.chunk_manager.depth;
  for index in (0..local_res.load_keys.len()) {
    let key = &local_res.load_keys.pop().unwrap();

    // if *key != [0, -1, 2] {
    if *key != [0, -1, 0] && *key != [-1, -1, 0] {
      // continue;
    }

    let chunk = res.chunk_manager.new_chunk3(key, lod as u8);
    local_res.chunks.push(chunk);
  }
}


fn add_meshes(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,

  mut local_res: ResMut<LocalResource>,
  mut res: ResMut<GameResource>,
) {
  let lod = res.chunk_manager.depth;
  let mut test_index = 0;
  for index in (0..local_res.load_mesh_keys.len()).rev() {
    let key = &local_res.load_mesh_keys[index].clone();
    let chunk_op = get_chunk(key, &local_res.chunks);
    if chunk_op.is_none() {
      continue;
    }
    local_res.load_mesh_keys.swap_remove(index);

    let chunk = chunk_op.unwrap();
    if !is_valid_chunk(&chunk) {
      // continue;
    }

    // if *key != [0, -1, 2] {
    //   continue;
    // }
    
    let d = chunk.octree.compute_mesh2(VoxelMode::SurfaceNets);
    if d.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
      continue;
    }
    // info!("d.indices.len() {}", d.indices.len());
    let mesh = create_mesh(&mut meshes, d.positions, d.normals, d.uvs, d.indices);

    let seamless_size = res.chunk_manager.seamless_size();
    let coord_f32 = key_to_world_coord_f32(key, seamless_size);

    commands
      .spawn_bundle(PbrBundle {
        mesh: mesh,
        material: materials.add(Color::rgba(0.5, 0.4, 0.3, 0.3).into()),
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
        ..Default::default()
      });

    test_index += 1;
  }
}

fn add_colliders(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,

  mut local_res: ResMut<LocalResource>,
  mut res: ResMut<GameResource>,
) {
  let keys = local_res.load_collider_keys.clone();
  for index in (0..keys.len()).rev() {
    let key = &keys[index];    
    let chunk_op = get_chunk(key, &local_res.chunks);
    if chunk_op.is_none() {
      continue;
    }
    local_res.load_collider_keys.swap_remove(index);

    let chunk = chunk_op.unwrap();
    if !is_valid_chunk(&chunk) {
      continue;
    }

    let data = create_collider_mesh(&chunk.octree);
    if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
      continue;
    }

    let seamless_size = res.chunk_manager.seamless_size();
    let pos_f32 = key_to_world_coord_f32(key, seamless_size);

    commands
      .spawn()
      .insert(Collider::trimesh(data.positions.clone(), data.indices.clone()))
      .insert(Transform::from_xyz(pos_f32[0], pos_f32[1], pos_f32[2]) )
      .insert(GlobalTransform::default());

    // info!("collider created key {:?}", key);
  }
}

fn remove_colliders(
  res: Res<GameResource>,

  mut commands: Commands,
  colliders: Query<(Entity, &Transform, &Collider), Without<Character>>,
  chars: Query<(&Character)>,
) {
  for char in chars.iter() {
    let adj_keys = adjacent_keys(&char.cur_key, 1);

    for (entity, transform, collider) in colliders.iter() {
      let collider_key = to_key(&transform.translation, res.chunk_manager.seamless_size());

      if !adj_keys.contains(&collider_key) {
        commands.entity(entity).despawn_recursive();
      }
    }
    


  }
}









fn create_collider_mesh(octree: &VoxelOctree) -> MeshColliderData {
  let mesh = get_surface_nets2(octree);

  let mut positions = Vec::new();
  let mut indices = Vec::new();
  
  for pos in mesh.positions.iter() {
    // positions.push(Point::new(pos[0], pos[1], pos[2]));
    positions.push(Vec3::new(pos[0], pos[1], pos[2]));
  }
  
  for ind in mesh.indices.chunks(3) {
    // println!("i {:?}", ind);
    indices.push([ind[0], ind[1], ind[2]]);
  }


  MeshColliderData {
    positions: positions,
    indices: indices,
  }
}


fn key_to_world_coord_f32(key: &[i64; 3], seamless_size: u32) -> [f32; 3] {
  [
    (key[0] * seamless_size as i64) as f32,
    (key[1] * seamless_size as i64) as f32,
    (key[2] * seamless_size as i64) as f32,
  ]
}


fn get_chunk(key: &[i64; 3], chunks: &Vec<Chunk>) -> Option<Chunk> {
  for chunk in chunks.iter() {
    if chunk.key == *key {
      return Some(chunk.clone());
    }
  }
  None
}

fn is_valid_chunk(chunk: &Chunk) -> bool {
  chunk.mode == ChunkMode::Loaded
}


struct LocalResource {
  first_time_load: bool,
  load_keys: Vec<[i64; 3]>,
  load_mesh_keys: Vec<[i64; 3]>,
  load_collider_keys: Vec<[i64; 3]>,
  chunks: Vec<Chunk>
}

impl Default for LocalResource {
  fn default() -> Self {
    LocalResource {
      first_time_load: true,
      load_keys: Vec::new(),
      load_mesh_keys: Vec::new(),
      load_collider_keys: Vec::new(),
      chunks: Vec::new(),
    }
  }
}


#[derive(Clone)]
pub struct MeshColliderData {
  // pub positions: Vec<Point<f32>>,
  pub positions: Vec<Vec3>,
  pub indices: Vec<[u32; 3]>,
}




