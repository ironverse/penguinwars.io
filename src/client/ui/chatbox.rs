use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Frame, Color32};
use bevy_egui::egui::Rect;
use super::utils::{style::setup_style, new_window};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(startup)
      // .add_system(ui_example)
      .add_system(update);
  }
}

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  // let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

}

fn ui_example(mut ctx: ResMut<EguiContext>) {
  // egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
  //     ui.label("world");
  // });

  let s = setup_style(ctx.ctx_mut());
  // set_background_galaxy(ctx.ctx_mut(), &s);

  //panel
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(76, 67, 82, 00),
    ..Default::default()
  };

  // new_window(ctx.ctx_mut(), &s, "signup_window", frame, |ui| {
  //   //panel contents
  //   ui.vertical_centered(|ui| {
  //     ui.add_space(s.y(30.0));
  //     //heading
  //     ui.heading("Create Your Ironverse Account");


  //     // ui.image(texture_id, size)
  //   });
  // });

  // egui::Window::new("Window")
  //   .vscroll(true)
  //   .show(egui_ctx.ctx_mut(), |ui| {
  //       ui.label("Windows can be moved by dragging them.");
  //       ui.label("They are automatically sized based on contents.");
  //       ui.label("You can turn on resizing and scrolling if you like.");
  //       ui.label("You would normally chose either panels OR windows.");
  //   });
}

fn update() {

}
