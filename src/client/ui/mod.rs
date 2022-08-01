use bevy::prelude::*;
use super::{camera::CameraSettings, char::Character};
use bevy_egui::EguiPlugin;

mod bubble;
mod chatbox;
mod utils;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(EguiPlugin)
      .add_plugin(bubble::CustomPlugin)
      .add_plugin(chatbox::CustomPlugin);
  }
}

