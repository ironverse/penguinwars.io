use bevy::{prelude::*, utils::Instant, tasks::{AsyncComputeTaskPool, Task}};
use bevy_rapier3d::{rapier::{prelude::{ColliderFlags, ColliderShape, ColliderPosition, ActiveCollisionTypes}, math::Point}, prelude::Collider};
use voxels::{chunk::{adjacent_keys_by_dist, chunk_manager::{ChunkMode, Chunk}, adjacent_keys, delta_keys, adj_delta_keys, world_pos_to_key, in_range, adjacent_keys_minmax, delta_keys_minmax}, data::{voxel_octree::{VoxelMode, VoxelOctree, MeshData}, surface_nets::{get_surface_nets2, VoxelReuse}}};
use crate::utils::to_key;
use super::{GameResource, utils::create_mesh, char::Character};
use voxels::chunk::chunk_manager::ChunkManager;
use futures_lite::future;
use bevy::core::FixedTimestep;

const LOWEST_TIME_DELTA_LIMIT: f32 = 1.0 / 55.0;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default());
    app
      .add_system_to_stage(CoreStage::First, reset_time)
      .add_system(entered_world_keys)
      .add_system(movement_delta_keys)
      .add_system(queue_chunk)
      .add_system(loaded_chunk)
      .add_system(loaded_mesh_data)
      .add_system(queue_collider_data)
      .add_system(loaded_collider_data)
      .add_system(remove_meshes)
      .add_system(remove_colliders)
      .add_system(remove_data_cache)
      ;

    app
      .add_system_set(
        SystemSet::new()
          .with_run_criteria(FixedTimestep::step(1.0))
          .with_system(log),
    );
  }
}


fn reset_time(mut local_res: ResMut<LocalResource>) {
  local_res.delta_time = 0.0;
}

fn entered_world_keys(
  res: Res<GameResource>,
  mut local_res: ResMut<LocalResource>,

  chars: Query<&Character, Added<Character>>,
) {
  for char in chars.iter() {
    for (index, dist) in res.chunk_manager.lod_dist.iter().enumerate() {
      let min = if index == 0 { 0 } else { res.chunk_manager.lod_dist[index - 1] };
      let max = res.chunk_manager.lod_dist[index];
      let keys = adjacent_keys_minmax(&char.cur_key, min, max);
      local_res.load_keys.push(keys.clone());
      local_res.load_mesh_keys.push(keys.clone());

      local_res.chunks.push(Vec::new());
    }

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

    for (index, dist) in res.chunk_manager.lod_dist.iter().enumerate() {
      let min = if index == 0 { 0 } else { res.chunk_manager.lod_dist[index - 1] };
      let max = res.chunk_manager.lod_dist[index];
      let keys = delta_keys_minmax(&char.prev_key, &char.cur_key, min, max);
      local_res.load_keys[index].extend(keys.clone());
      local_res.load_mesh_keys[index].extend(keys.clone());
    }

    let col_keys = adj_delta_keys(&char.prev_key, &char.cur_key, 1);
    local_res.load_collider_keys.extend(col_keys.iter());
  }
}

fn queue_chunk(
  mut commands: Commands, 
  thread_pool: Res<AsyncComputeTaskPool>,
  mut local_res: ResMut<LocalResource>,
  mut res: ResMut<GameResource>,
) {

  for lod_index in (0..local_res.load_keys.len()) {
    for index in (0..local_res.load_keys[lod_index].len()).rev() {
      let key = local_res.load_keys[lod_index].swap_remove(index);

      let lod = res.chunk_manager.depth - lod_index as u32;
      let depth = res.chunk_manager.depth;
      let noise = res.chunk_manager.noise.clone();

      let task = thread_pool.spawn(async move {

        // Transform::from_xyz(x as f32, y as f32, z as f32)
        let chunk = ChunkManager::new_chunk(&key, depth as u8, lod as u8, noise);

        ChunkData {
          lod_index: lod_index,
          lod: lod as u8,
          chunk: chunk,
        }
      });

      // Spawn new entity and add our new task as a component
      commands.spawn().insert(task);
    }
  }
}

fn loaded_chunk(
  mut commands: Commands,
  mut chunk_tasks: Query<(Entity, &mut Task<ChunkData>)>,
  mut res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

  thread_pool: Res<AsyncComputeTaskPool>,
) {
  for (entity, mut task) in chunk_tasks.iter_mut() {
    if let Some(data) = future::block_on(future::poll_once(&mut *task)) {
      local_res.chunks[data.lod_index].push(data.chunk.clone());

      let lod_index = data.lod_index;
      let key = data.chunk.key.clone();

      // Queue MeshData
      let mut voxel_reuse = res.chunk_manager.voxel_reuse.clone();
      let task = thread_pool.spawn(async move {
        MeshChunkData {
          lod_index: lod_index,
          key: key,
          data: data.chunk.octree.compute_mesh2(VoxelMode::SurfaceNets, &mut voxel_reuse)
        }
      });

      commands.spawn().insert(task);

      // Task is complete, so remove task component from entity
      commands.entity(entity).remove::<Task<ChunkData>>();
    }
  }
}

fn loaded_mesh_data(
  mut commands: Commands,
  mut chunk_tasks: Query<(Entity, &mut Task<MeshChunkData>)>,
  mut res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  for (entity, mut task) in chunk_tasks.iter_mut() {
    if let Some(mc_data) = future::block_on(future::poll_once(&mut *task)) {
      let lod_index = mc_data.lod_index;
      let data = mc_data.data;

      let len = data.indices.len();
      if len != 0 { // Temporary, should be removed once the ChunkMode detection is working
        let mesh = create_mesh(&mut meshes, data.positions, data.normals, data.uvs, data.indices);
    
        let seamless_size = res.chunk_manager.seamless_size();
        let coord_f32 = key_to_world_coord_f32(&mc_data.key, seamless_size);

        commands
          .spawn_bundle(PbrBundle {
            mesh: mesh,
            material: materials.add(Color::rgba(0.5, 0.4, 0.3, 0.3).into()),
            transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
            ..Default::default()
          })
          .insert(TerrainChunk {
            lod_index: lod_index
          });
      }


      // Task is complete, so remove task component from entity
      commands.entity(entity).remove::<Task<MeshChunkData>>();
    }
  }
}

fn queue_collider_data(
  mut commands: Commands,
  mut local_res: ResMut<LocalResource>,
  mut res: ResMut<GameResource>,
  thread_pool: Res<AsyncComputeTaskPool>,
) {
  let keys = local_res.load_collider_keys.clone();
  for index in (0..keys.len()).rev() {
    let key = keys[index];    
    let chunk_op = get_chunk(&key, &local_res.chunks[0]);
    if chunk_op.is_none() {
      continue;
    }
    local_res.load_collider_keys.swap_remove(index);

    let chunk = chunk_op.unwrap();
    if !is_valid_chunk(&chunk) {
      continue;
    }

    let mut voxel_reuse = res.chunk_manager.voxel_reuse.clone();
    let task = thread_pool.spawn(async move {
      let data = create_collider_mesh(&chunk.octree, &mut voxel_reuse);
      
      ColliderData {
        key: key.clone(),
        data: data,
      }
    });
    commands.spawn().insert(task);
  }
}

fn loaded_collider_data(
  mut commands: Commands,
  mut chunk_tasks: Query<(Entity, &mut Task<ColliderData>)>,

  mut res: ResMut<GameResource>,
) {
  for (entity, mut task) in chunk_tasks.iter_mut() {
    if let Some(c_data) = future::block_on(future::poll_once(&mut *task)) {
      commands.entity(entity).remove::<Task<ColliderData>>();

      let key = c_data.key;
      let data = c_data.data;

      if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
        continue;
      }

      let seamless_size = res.chunk_manager.seamless_size();
      let pos_f32 = key_to_world_coord_f32(&key, seamless_size);

      commands
        .spawn()
        .insert(Collider::trimesh(data.positions.clone(), data.indices.clone()))
        .insert(Transform::from_xyz(pos_f32[0], pos_f32[1], pos_f32[2]) )
        .insert(GlobalTransform::default());
    }
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

fn remove_meshes(
  mut commands: Commands,
  res: Res<GameResource>,

  chars: Query<(&Character)>,
  terrain_query: Query<(Entity, &Transform, &TerrainChunk)>
) {
  for char in chars.iter() {
    for (entity, transform, terrain_chunk) in terrain_query.iter() {
      let key = to_key(&transform.translation, res.chunk_manager.seamless_size());
      
      for (index, dist) in res.chunk_manager.lod_dist.iter().enumerate() {

        let min = if index == 0 { 0 } else { res.chunk_manager.lod_dist[index - 1] };
        let max = dist;

        if terrain_chunk.lod_index == index && 
        !in_range(&char.cur_key, &key, *max) {
          commands.entity(entity).despawn_recursive();
        }

        if index != 0 {
          if terrain_chunk.lod_index == index && in_range(&char.cur_key, &key, min) {
            commands.entity(entity).despawn_recursive();
          }
        }
        
      }
      
    }
  }
  
}

fn remove_data_cache(
  mut local_res: ResMut<LocalResource>,
  res: Res<GameResource>,

  chars: Query<(&Character)>,
) {

  let mut player_key = [0; 3];
  // FIXME: Should specify the player only, not other player
  for char in chars.iter() {
    player_key = char.cur_key.clone();
  }

  for lod_index in (0..local_res.chunks.len()) {
    for index in (0..local_res.chunks[lod_index].len()).rev() {
      // let key = local_res.load_keys[lod_index].swap_remove(index);
      let chunk = &local_res.chunks[lod_index][index];

      let dist = res.chunk_manager.lod_dist[lod_index];

      if !in_range(&player_key, &chunk.key, dist) {
        local_res.chunks[lod_index].swap_remove(index);
      }

    }
  }
}



fn log(local_res: Res<LocalResource>,) {
  let mut count = 0;
  for index in 0..local_res.chunks.len() {
    count += local_res.chunks[index].len();
  }
  println!("chunks cache count {:?}", count);
}


// Helper functions
fn create_collider_mesh(octree: &VoxelOctree, voxel_reuse: &mut VoxelReuse) -> MeshColliderData {
  let mesh = get_surface_nets2(octree, voxel_reuse);

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
  load_keys: Vec<Vec<[i64; 3]>>,
  load_mesh_keys: Vec<Vec<[i64; 3]>>,
  load_collider_keys: Vec<[i64; 3]>,
  chunks: Vec<Vec<Chunk>>,
  delta_time: f32,
}

impl Default for LocalResource {
  fn default() -> Self {
    LocalResource {
      first_time_load: true,
      load_keys: Vec::new(),
      load_mesh_keys: Vec::new(),
      load_collider_keys: Vec::new(),
      chunks: Vec::new(),
      delta_time: 0.0,
    }
  }
}


#[derive(Clone)]
pub struct MeshColliderData {
  // pub positions: Vec<Point<f32>>,
  pub positions: Vec<Vec3>,
  pub indices: Vec<[u32; 3]>,
}

#[derive(Component)]
pub struct TerrainChunk {
  pub lod_index: usize
}

pub struct ChunkData {
  pub lod_index: usize,
  pub lod: u8,
  pub chunk: Chunk
}

pub struct MeshChunkData {
  pub lod_index: usize,
  pub key: [i64; 3],
  pub data: MeshData
}

pub struct ColliderData {
  pub key: [i64; 3],
  pub data: MeshColliderData
}