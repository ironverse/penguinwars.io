use bevy::prelude::*;
use voxel::{chunk::adjacent_keys_by_dist, data::voxel_octree::VoxelMode};
use super::{GameResource, utils::create_mesh};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default());

    app
      .add_startup_system(entered_world_keys);

    app
      .add_system(movement_delta_keys)
      .add_system(load_mesh);
  }
}

fn entered_world_keys(
  mut res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,
) {
  let lod0 = res.chunk_manager.client_lod0;
  let keys0 = adjacent_keys_by_dist(&[0, 0, 0], lod0);

  local_res.keys = keys0;
  for key in local_res.keys.iter() {
    // info!("key {:?}", key);
  }
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