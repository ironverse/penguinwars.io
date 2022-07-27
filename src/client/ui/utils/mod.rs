pub mod style;

use bevy_egui::egui;
use bevy_egui::egui::Frame;
use bevy_egui::egui::Rect;
use bevy_egui::egui::Ui;
use bevy_egui::egui::WidgetText;
use bevy_egui::egui::Window;

use self::style::Scaler;

pub const AUTH_PANEL_MIN: [f32; 2] = [600.0, 305.0];
pub const AUTH_PANEL_MAX: [f32; 2] = [1320.0, 774.0];

pub fn new_window(
  ctx: &egui::Context,
  s: &Scaler,
  title: impl ToString,
  frame: Frame,
  add_contents: impl FnOnce(&mut Ui),
) {
  let mut e: Window = egui::Window::new(WidgetText::from(title.to_string().as_str()));

  let test = s.testing(AUTH_PANEL_MIN);

  e.title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: s.xy(AUTH_PANEL_MIN).into(),
      max: s.xy(AUTH_PANEL_MAX).into(),
    })
    .show(ctx, |ui| {
      ui.set_min_height(s.y(AUTH_PANEL_MAX[1] - AUTH_PANEL_MIN[1]).into());
      add_contents(ui);
    });
}