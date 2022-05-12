pub mod client;
pub mod server;
pub mod utils;
use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_rapier3d::{prelude::*, rapier::prelude::IntegrationParameters};
use voxels::chunk::chunk_manager::ChunkManager;

fn main() {
  let mut app = App::new();
  app
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    // .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .insert_resource(IntegrationParameters {
      dt: 1.0 / 30.0,
      ..Default::default()
    })
    
    .add_plugin(client::CustomPlugin)
    .add_plugin(server::CustomPlugin);
    // .run();

  // #[cfg(not(target_arch = "wasm32"))]
  // app
  //   .add_plugin(RapierDebugRenderPlugin::default());
  
  app.run();
}


/*
  TODO
    There is a stutter when creating terrain
      What are the possible causes?
        Loading of data
        Creation of mesh
        Creation of collider
        Initialization of colliders in bevy_rapier3d
        Initialization of bevy for mesh
*/