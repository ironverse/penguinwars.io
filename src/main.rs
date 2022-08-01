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
    .insert_resource(WindowDescriptor {
      title: "Penguin Wars".to_string(),
      width: 1280.,
      height: 720.,
      // present_mode: PresentMode::AutoVsync,
      ..default()
    })
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
    Rendered mesh disposal when out of range
    Remove chunk data when out of range
    Bubble message in 3D world
    Player name in 3D world
*/