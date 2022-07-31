use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Frame, Color32};
use bevy_egui::egui::Rect;
use super::utils::{style::setup_style, new_window};

pub struct ChatResource {
  pub text: String,
  pub is_chat_mode: bool
}

pub struct ChatEvent {
  pub text: String,
}


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ChatResource {
        is_chat_mode: false,
        text: "".to_string(),
      })
      .add_event::<ChatEvent>()
      .add_startup_system(startup)
      // .add_system(ui_example)
      .add_system(key_events)
      .add_system(update);
  }
}

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  // let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
}

fn key_events(
  keyboard_input: Res<Input<KeyCode>>,
  mut chat_res: ResMut<ChatResource>,
  mut chat_events: EventWriter<ChatEvent>,
) {
  if keyboard_input.just_pressed(KeyCode::Return) {

    if chat_res.text.len() > 0 {
      chat_events.send(ChatEvent {
        text: chat_res.text.to_string(),
      });
      chat_res.text = "".to_string();
    }
    chat_res.is_chat_mode = true;
    
    // chat_res.is_chat_mode = !chat_res.is_chat_mode;
    // if !chat_res.is_chat_mode {
    //   // bubble_res.text = chat_res.text.to_string();
    //   chat_events.send(ChatEvent {
    //     text: chat_res.text.to_string(),
    //   });
    //   chat_res.text = "".to_string();
    // }
    // println!("'Return' currently pressed {:?}", chat_res.is_chat_mode);
  }
}

fn update(
  mut ctx: ResMut<EguiContext>,
  windows: Res<Windows>,
  mut chat_res: ResMut<ChatResource>,
) {
  let s = setup_style(ctx.ctx_mut());

  //panel
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 125, 255),
    ..Default::default()
  };

  let dim = egui::Vec2::new(600.0, 100.0);
  let win_height = windows.get_primary().unwrap().height();
  let pos = egui::Pos2::new(0.0, win_height - dim.y as f32);
  
  let size = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), dim);

  new_window(ctx.ctx_mut(), &s, "Chat Box", frame, pos, size, |ui| {
    /*
      TODO
        Input text
        History of text
          At least 4 text history
    */
    
    // ui.label("world");
    if chat_res.is_chat_mode {
      let r = ui.text_edit_singleline(&mut chat_res.text);
      r.request_focus();
      if r.clicked_elsewhere() {
        chat_res.is_chat_mode = false;
        println!("Clicked elsewhere");
        r.surrender_focus();
      }
    } else {
      if ui.text_edit_singleline(&mut chat_res.text).clicked() {
        chat_res.is_chat_mode = true;
        println!("Clicked");
      }
    }
    
  });
}