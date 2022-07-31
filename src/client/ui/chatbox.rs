use bevy::prelude::*;
use bevy_egui::egui::style::Margin;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Frame, Color32};
use bevy_egui::egui::Rect;
use super::utils::history_window;
use super::utils::{style::setup_style, new_window};

const HISTORY_LIMIT: usize = 10;
const WIDTH: f32 = 500.0;

pub struct ChatResource {
  pub text: String,
  pub is_chat_mode: bool,
  pub history: Vec<String>,
  scroll_to_last_msg: bool,
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
        history: vec!["".into(); HISTORY_LIMIT],
        scroll_to_last_msg: false,
      })
      .add_event::<ChatEvent>()
      .add_startup_system(startup)
      // .add_system(ui_example)
      .add_system(key_events)
      .add_system(chat_box)
      .add_system(history);
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
      let msg = chat_res.text.to_string();
      chat_events.send(ChatEvent {
        text: msg.clone(),
      });

      if chat_res.history.len() > HISTORY_LIMIT {
        chat_res.history.remove(0);
      }
      chat_res.history.push(msg.clone());


      chat_res.text = "".to_string();
      chat_res.scroll_to_last_msg = true;
    }
    chat_res.is_chat_mode = true;
  }
}

fn chat_box(
  mut ctx: ResMut<EguiContext>,
  windows: Res<Windows>,
  mut chat_res: ResMut<ChatResource>,
) {
  let s = setup_style(ctx.ctx_mut());

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 127),
    ..Default::default()
  };

  let dim = egui::Vec2::new(WIDTH, 20.0);
  let win_height = windows.get_primary().unwrap().height();
  let pos = egui::Pos2::new(0.0, win_height - dim.y as f32);
  let size = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), dim);

  new_window(ctx.ctx_mut(), &s, "Chat Box", frame, pos, size, |ui| {
    

    if chat_res.is_chat_mode {
      let r = ui.add_sized(dim, egui::TextEdit::singleline(&mut chat_res.text));
      r.request_focus();
      if r.clicked_elsewhere() {
        chat_res.is_chat_mode = false;
        r.surrender_focus();
      }
    } else {
      if ui.add_sized(dim, egui::TextEdit::singleline(&mut chat_res.text)).clicked() {
        chat_res.is_chat_mode = true;
      }
    }
    
  });
}

fn history(
  mut ctx: ResMut<EguiContext>,
  windows: Res<Windows>,
  mut chat_res: ResMut<ChatResource>,
) {
  let s = setup_style(ctx.ctx_mut());
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 127),
    margin: Margin {
      left: 5.0,
      right: 5.0,
      top: 5.0,
      bottom: 5.0,
    },
    ..Default::default()
  };

  let scroll_width = 26.0;
  let dim = egui::Vec2::new(WIDTH - scroll_width, 80.0);
  let win_height = windows.get_primary().unwrap().height();

  let adj_y = 20.0 + dim.y + frame.margin.bottom + frame.margin.top;
  let pos = egui::Pos2::new(0.0, win_height - adj_y);
  let size = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), dim);

  let text_size = egui::Vec2::new(dim.x, 20.0);

  let scroll_to_last_msg = chat_res.scroll_to_last_msg;
  if chat_res.scroll_to_last_msg {
    chat_res.scroll_to_last_msg = false;
  }
  history_window(ctx.ctx_mut(), &s, "History", frame, pos, size, |ui| {
    for h in chat_res.history.iter() {
      let r = ui.add_sized(text_size, 
        egui::Label::new(
          egui::RichText::new(h.to_string())
            .font(egui::FontId {
              size: 15.0,
              // family: egui::FontFamily::Name("Medium".into())
              family: egui::FontFamily::Proportional,
            })
            .color(Color32::from_rgba_premultiplied(255, 255, 255, 255)
        )
      ));

      if scroll_to_last_msg {
        r.scroll_to_me(Some(egui::Align::Center));
      }
    }
  });
}

    /*
      TODO
        Input text
        History of text
          At least 4 text history
    */