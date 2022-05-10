pub mod client;
pub mod server;
pub mod utils;
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::IntegrationParameters};
use voxels::chunk::chunk_manager::ChunkManager;

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .insert_resource(IntegrationParameters {
      dt: 1.0 / 30.0,
      ..Default::default()
    })
    
    .add_plugin(client::CustomPlugin)
    .add_plugin(server::CustomPlugin)
    .run();
}


/*
  TODO
    Player movement using bevy_rapier3d
    Terrain collider
*/