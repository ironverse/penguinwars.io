use bevy::prelude::*;
use super::{camera::CameraSettings, char::Character};

mod bubble;
mod utils;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(bubble::CustomPlugin)
      .add_startup_system(startup)
      .add_system_to_stage(CoreStage::First, update); // Need to be first to remove positioning stutter
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
        "My Name is Wut?".to_string(),
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

#[derive(Component)]
struct FollowText;