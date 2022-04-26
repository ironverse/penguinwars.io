use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(startup);
  }
}

fn startup() {
  info!("testing");
}