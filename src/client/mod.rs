pub mod camera;
pub mod char;
pub mod terrain;
pub mod utils;
pub mod debugger;

use bevy::prelude::*;
use voxels::chunk::chunk_manager::ChunkManager;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameResource::default());

    app
      .add_plugin(camera::CustomPlugin)
      .add_plugin(char::CustomPlugin)
      .add_plugin(terrain::CustomPlugin)
      .add_plugin(debugger::CustomPlugin);
  }
}

pub struct GameResource {
  pub chunk_manager: ChunkManager
}

impl Default for GameResource {
  fn default() -> Self {
    GameResource {
      chunk_manager: ChunkManager::default()
    }
  }
}
