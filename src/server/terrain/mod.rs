pub mod data;
use bevy::prelude::*;
use voxels::chunk::chunk_manager::ChunkManager;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    // app
    //   .insert_resource(GameResource::default());

    app
      .add_plugin(data::CustomPlugin);

    
  }
}


// pub struct GameResource {
//   pub chunk_manager: ChunkManager
// }

// impl Default for GameResource {
//   fn default() -> Self {
//     GameResource {
//       chunk_manager: ChunkManager::default()
//     }
//   }
// }

