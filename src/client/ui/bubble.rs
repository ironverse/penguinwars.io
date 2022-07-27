use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};

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


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(startup)
      .add_system_to_stage(CoreStage::First, update)
      .add_system(update_bubble)
      ; // Need to be first to remove positioning stutter
  }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  commands
    .spawn_bundle(TextBundle {
      style: Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
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
        TextStyle {
          font,
          font_size: 20.0,
          color: Color::WHITE,
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

use bevy_egui::egui::{Frame, Color32};

use super::utils::{new_window, style::setup_style};
fn update_bubble(
  mut ctx: ResMut<EguiContext>,
  mut rendered_texture_id: Local<egui::TextureId>,
  mut local: Local<Cache>,
  images: Local<Images>,
) {

  if !local.initilized {
    local.initilized = true;
    *rendered_texture_id = ctx.add_image(images.bubble.clone_weak());
  }

  let s = setup_style(ctx.ctx_mut());
  // set_background_galaxy(ctx.ctx_mut(), &s);

  //panel
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(76, 67, 82, 00),
    ..Default::default()
  };

  new_window(ctx.ctx_mut(), &s, "Game Window", frame, |ui| {
    let image = egui::widgets::Image::new(
      *rendered_texture_id,
      [256.0, 256.0],
    );

    ui.add(image);
  });
}





/*
  TODO:
    Make the bubble and text follow the player

    Event when player is created
*/