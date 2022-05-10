use bevy::prelude::*;
use bevy_rapier3d::{rapier::{prelude::{ColliderFlags, ColliderShape, ColliderPosition, ActiveCollisionTypes}, math::Point}, prelude::Collider};
use voxels::{chunk::adjacent_keys_by_dist, data::{voxel_octree::{VoxelMode, VoxelOctree}, surface_nets::{get_surface_nets2}}};

use super::{GameResource, utils::create_mesh};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default());
    // app
    //   .add_startup_system(entered_world_keys);
    app
      .add_system(movement_delta_keys)
      .add_system(load_mesh);
  }
}

fn entered_world_keys(
  res: Res<GameResource>,
  mut local_res: ResMut<LocalResource>,
) {
  // let lod0 = res.chunk_manager.client_lod0;
  let keys0 = adjacent_keys_by_dist(&[0, 0, 0], 2);

  local_res.keys.extend(keys0.iter());
}

fn movement_delta_keys() {

}


fn load_mesh(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,

  mut local_res: ResMut<LocalResource>,
  mut res: ResMut<GameResource>,

  
) {
  /* 
    Load all
    Load based on time, copy from the prototype
  */
  let lod = res.chunk_manager.depth;
  for index in (0..local_res.keys.len()) {
    let key = &local_res.keys.pop().unwrap();

    let chunk = res.chunk_manager.new_chunk3(key, lod as u8);
    let d = chunk.octree.compute_mesh2(VoxelMode::SurfaceNets);

    if d.positions.len() == 0 {
      continue;
    }
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
      // .insert(Terrain {
      //   id: res.chunk_id,
      //   key: key3.clone(),
      //   ttl: 0.0,
      //   dispose: false,
      //   lod: key[3] as u32,
      //   loaded_type: LoadedType::Default,
      // });
    spawn_collider(&mut commands, &chunk.octree, key, seamless_size);
  }
}


fn spawn_collider(
  commands: &mut Commands,
  octree: &VoxelOctree, 
  key: &[i64; 3], 
  seamless_size: u32
) {
  let data = create_collider_mesh(octree);
  let pos_f32 = key_to_world_coord_f32(key, seamless_size);

  commands
    .spawn()
    // .insert(Collider::trimesh(data.positions.clone(), data.indices.clone()))
    .insert(Transform::from_xyz(pos_f32[0], pos_f32[1], pos_f32[2]) )
    .insert(GlobalTransform::default())
    ;



  // let collider = ColliderBundle {
  //   position: ColliderPosition(pos_f32.into()),
  //   shape: ColliderShape::trimesh(data.positions.clone(), data.indices.clone()),
  //   flags: ColliderFlags {
  //     // collision_groups: InteractionGroups::new(res.loaded_id, u32::MAX),
  //     active_collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
  //     ..ColliderFlags::default()
  //   },
  //   ..ColliderBundle::default()
  // };

  // commands
  //   .spawn_bundle(collider);
    // .insert(ColliderChunk { 
    //   key: key.clone(), delete: false, collider_type: ColliderChunkType::Terrain 
    // });
}


pub fn create_collider_mesh(octree: &VoxelOctree) -> MeshColliderData {
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




struct LocalResource {
  keys: Vec<[i64; 3]>
}

impl Default for LocalResource {
  fn default() -> Self {
    LocalResource {
      keys: Vec::new(),
    }
  }
}

#[derive(Clone)]
pub struct MeshColliderData {
  // pub positions: Vec<Point<f32>>,
  pub positions: Vec<Vec3>,
  pub indices: Vec<[u32; 3]>,
}




