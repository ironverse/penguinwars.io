pub mod camera;
pub mod char;

use bevy::prelude::*;


/*
  TODO:
    Define the scope of what this will handle
*/

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(camera::CustomPlugin)
      .add_plugin(char::CustomPlugin)
      .add_startup_system(startup);
  }
}

fn startup() {
  info!("testing");
}