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
    Implement server side physics
    Simplify the chunk data to be used on both server and client
    Server should be the one sending data to the client
    Server will receive data from the connector(Defer)


    v1
      We should separate the server from the client
      Connect them via GlobalResource for now
      All updates will be from server
      Rendering all on client

*/