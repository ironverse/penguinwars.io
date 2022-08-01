pub mod style;

use bevy_egui::egui;
use bevy_egui::egui::Frame;
use bevy_egui::egui::Pos2;
use bevy_egui::egui::Rect;
use bevy_egui::egui::Ui;
use bevy_egui::egui::WidgetText;
use bevy_egui::egui::Window;

use self::style::Scaler;

pub const AUTH_PANEL_MIN: [f32; 2] = [600.0, 305.0];
pub const AUTH_PANEL_MAX: [f32; 2] = [1320.0, 774.0];

pub const SIZE_MIN: [f32; 2] = [100.0, 50.0];
pub const SIZE_MAX: [f32; 2] = [1920.0, 1080.0];

pub fn new_window(
  ctx: &egui::Context,
  s: &Scaler,
  title: impl ToString,
  frame: Frame,
  pos: Pos2,
  size: Rect,
  add_contents: impl FnOnce(&mut Ui),
) {
  let mut e: Window = egui::Window::new(WidgetText::from(title.to_string().as_str()));

  e.title_bar(false)
    .frame(frame)
    .fixed_rect(size)
    .current_pos(pos)
    .show(ctx, |ui| {
      ui.set_width(size.width());
      ui.set_height(size.height());
      // ui.set_min_height(s.y(AUTH_PANEL_MAX[1] - AUTH_PANEL_MIN[1]).into());
      add_contents(ui);
    });
}

pub fn history_window(
  ctx: &egui::Context,
  s: &Scaler,
  title: impl ToString,
  frame: Frame,
  pos: Pos2,
  size: Rect,
  add_contents: impl FnOnce(&mut Ui),
) {
  let mut e: Window = egui::Window::new(WidgetText::from(title.to_string().as_str()));

  e.title_bar(false)
    .frame(frame)
    .fixed_rect(size)
    .current_pos(pos)
    .vscroll(true)
    .show(ctx, |ui| {
      ui.set_width(size.width());
      add_contents(ui);
    });
}