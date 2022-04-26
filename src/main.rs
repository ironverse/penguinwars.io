pub mod client;

use bevy::prelude::*;

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(client::CustomPlugin)
    .run();
}