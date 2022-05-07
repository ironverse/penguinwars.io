use bevy::prelude::*;

/*
  Third person camera
*/

pub mod third_person;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(third_person::CustomPlugin);
  }
}


