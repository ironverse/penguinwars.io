use bevy::prelude::*;
use bevy_egui::{egui::{self, Pos2}, EguiContext, EguiSettings};
use bevy_egui::egui::{Frame, Color32};
use super::{utils::{new_window, style::setup_style}};
use crate::client::{char::Character, camera::CameraSettings};

struct Images {
  bubble: Handle<Image>,
}

impl FromWorld for Images {
  fn from_world(world: &mut World) -> Self {
      let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
      Self {
        bubble: asset_server.load("bubble.png"),
      }
  }
}

struct Cache {
  initilized: bool
}

impl Default for Cache {
  fn default() -> Self {
    Cache {
      initilized: false
    }
  }
}

#[derive(Component)]
struct FollowText;


pub struct BubbleResource {
  pub text: String,
}

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(BubbleResource {
        text: "".to_string(),
      })
      .add_startup_system(startup)
      .add_system_to_stage(CoreStage::First, update) // Need to be first to remove positioning stutter
      .add_system(update_bubble)
      ; 
  }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");

  commands
    .spawn_bundle(TextBundle {
      style: Style {
        // align_self: AlignSelf::FlexEnd,
        // position_type: PositionType::Absolute,
        position: Rect {
          bottom: Val::Px(5.0),
          left: Val::Px(15.0),
          ..Default::default()
        },
        size: Size {
          width: Val::Px(200.0),
          ..Default::default()
        },
        ..Default::default()
      },
      text: Text::with_section(
        "Player Name".to_string(),
        // "Player Name Player Name Player Name Player Name Player Name Player Name".to_string(),
        TextStyle {
          font: font.clone_weak(),
          font_size: 20.0,
          color: Color::BLUE,
        },
        TextAlignment {
          ..Default::default()
        }
      ),
      ..Default::default()
    })
    .insert(FollowText);
}


fn update(
  windows: Res<Windows>,
  images: ResMut<Assets<Image>>,
  cam_query: Query<(&Camera, &GlobalTransform), With<CameraSettings>>,
  char_query: Query<&Transform, With<Character>>,
  mut text_query: Query<(&mut Style, &CalculatedSize), With<FollowText>>,
) {
  for (cam, cam_transform) in cam_query.iter() {
    for char_transform in char_query.iter() {
      for (mut style, calculated) in text_query.iter_mut() {
        let translation = char_transform.translation + Vec3::new(0.0, 1.2, 0.0);

        match cam.world_to_screen(&windows, &images, cam_transform, translation) {
          Some(coords) => {
            style.position.left = Val::Px(coords.x - calculated.size.width / 2.0);
            style.position.bottom = Val::Px(coords.y - calculated.size.height / 2.0);
          }
          None => {

          }
        }

      }
    }
  }
}

fn update_bubble(
  windows: Res<Windows>,
  images: ResMut<Assets<Image>>,
  cam_query: Query<(&Camera, &GlobalTransform), With<CameraSettings>>,
  char_query: Query<&Transform, With<Character>>,
  bubble_res: Res<BubbleResource>,


  mut ctx: ResMut<EguiContext>,
  mut rendered_texture_id: Local<egui::TextureId>,
  mut local: Local<Cache>,
  local_images: Local<Images>,
) {

  if !local.initilized {
    local.initilized = true;
    *rendered_texture_id = ctx.add_image(local_images.bubble.clone_weak());
  }

  let player_height = 1.0;
  let mut x = 0.0;
  let mut y = 0.0;
  for (cam, cam_transform) in cam_query.iter() {
    for char_transform in char_query.iter() {
      let translation = char_transform.translation + Vec3::new(0.0, player_height, 0.0);

      match cam.world_to_screen(&windows, &images, cam_transform, translation) {
        Some(coords) => {
          x = coords.x;

          let mid = windows.get_primary().unwrap().height() / 2.0;
          let difY = mid - coords.y;
          y = mid + difY;
        }
        None => {

        }
      }

      let win_height = windows.get_primary().unwrap().height();

      let s = setup_style(ctx.ctx_mut());
      // set_background_galaxy(ctx.ctx_mut(), &s);

      //panel
      let frame = Frame {
        fill: Color32::from_rgba_premultiplied(76, 67, 82, 127),
        ..Default::default()
      };

      let width = 200.0;
      let height = 100.0;

      let left = x - width * 0.5;
      let bottom = y - height;
      let pos = Pos2::new(left, bottom);
      let size = egui::Rect::from_min_size(Pos2::new(0.0, 0.0), egui::Vec2::new(width, height));

      new_window(ctx.ctx_mut(), &s, "Bubble Image", frame, pos, size, |ui| {
        ui.add(egui::widgets::Image::new(
          *rendered_texture_id,
          [width, height],
        ));
      });

      // let text_size = egui::Vec2::new(width - 10.0, height - 10.0);
      let text_size = egui::Vec2::new(170.0, 70.0);
      let text_rect = egui::Rect::from_min_size(Pos2::new(0.0, 0.0), text_size);

      let text_adj_x = text_size.x * 0.5;
      let text_adj_y = text_size.y + win_height * -0.1325;
      let text_x = x - text_size.x + text_adj_x;
      let text_y = y - text_size.y + text_adj_y; 
      let text_pos = egui::Pos2::new(text_x, text_y);

      let text_frame = Frame {
        fill: Color32::from_rgba_premultiplied(0, 0, 0, 127),
        ..Default::default()
      };
      new_window(ctx.ctx_mut(), &s, "Bubble Text", text_frame, text_pos, text_rect, |ui| {
        ui.add_sized(text_size, 
          egui::Label::new(egui::RichText::new(bubble_res.text.to_string()).color(Color32::BLACK)
        ));
      });

    }
  }




  
}





/*
  TODO:
    Make the bubble and text follow the player

    Event when player is created
*/