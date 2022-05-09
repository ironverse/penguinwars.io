use bevy::prelude::*;
use voxels::chunk::adjacent_keys_by_dist;

use super::GameResource;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(entered_world_keys);

    app
      .add_system(movement_delta_keys);
  }
}

fn entered_world_keys(mut res: ResMut<GameResource>) {
  let lod0 = res.chunk_manager.client_lod0;
  let keys0 = adjacent_keys_by_dist(&[0, 0, 0], lod0);
}

fn movement_delta_keys() {

}