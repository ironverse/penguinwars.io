pub mod client;
pub mod server;
pub mod utils;

use bevy::prelude::*;

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(client::CustomPlugin)
    .add_plugin(server::CustomPlugin)
    .run();
}