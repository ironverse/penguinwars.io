use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

use super::{terrain::TerrainChunk, char::Character};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup_text)
      .add_startup_system(mesh_count_text)
      .add_startup_system(player_pos_text)
      .add_system(update_text)
      .add_system(update_mesh_count)
      .add_system(update_player_pos);
  }
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(UiCameraBundle::default());

  commands.spawn_bundle(TextBundle {
    style: Style {
      align_self: AlignSelf::FlexStart,
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(5.0),
        left: Val::Px(15.0),
        ..default()
      },
      ..default()
    },
    // Use the `Text::with_section` constructor
    text: Text::with_section(
      // Accepts a `String` or any type that converts into a `String`, such as `&str`
      "hello\nbevy!",
      TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
      },
      // Note: You can use `Default::default()` in place of the `TextAlignment`
      TextAlignment {
        horizontal: HorizontalAlign::Center,
        ..default()
      },
    ),
    ..default()
})
  .insert(FpsText);
}

fn mesh_count_text(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(UiCameraBundle::default());

  commands.spawn_bundle(TextBundle {
    style: Style {
      align_self: AlignSelf::FlexStart,
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(25.0),
        left: Val::Px(15.0),
        ..default()
      },
      ..default()
    },
    // Use the `Text::with_section` constructor
    text: Text::with_section(
      // Accepts a `String` or any type that converts into a `String`, such as `&str`
      "Mesh count",
      TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
      },
      // Note: You can use `Default::default()` in place of the `TextAlignment`
      TextAlignment {
        horizontal: HorizontalAlign::Center,
        ..default()
      },
    ),
    ..default()
})
  .insert(MeshCountText);
}

fn player_pos_text(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(UiCameraBundle::default());

  commands.spawn_bundle(TextBundle {
    style: Style {
      align_self: AlignSelf::FlexStart,
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(45.0),
        left: Val::Px(15.0),
        ..default()
      },
      ..default()
    },
    // Use the `Text::with_section` constructor
    text: Text::with_section(
      // Accepts a `String` or any type that converts into a `String`, such as `&str`
      "Mesh count",
      TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
      },
      // Note: You can use `Default::default()` in place of the `TextAlignment`
      TextAlignment {
        horizontal: HorizontalAlign::Center,
        ..default()
      },
    ),
    ..default()
})
  .insert(PlayerPosText);
}


fn update_text(
  diagnostics: Res<Diagnostics>, 
  mut query: Query<&mut Text, With<FpsText>>
) {
  for mut text in query.iter_mut() {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(average) = fps.average() {
        text.sections[0].value = format!("FPS: {:.2}", average);
      }
    }
  }
}


fn update_mesh_count(
  diagnostics: Res<Diagnostics>, 
  mut query: Query<&mut Text, With<MeshCountText>>,
  mesh_query: Query<&TerrainChunk>
) {

  let mut count0 = 0;
  let mut count1 = 0;
  for terrain in mesh_query.iter() {
    if terrain.lod == 0 {
      count0 += 1;
    }

    if terrain.lod == 1 {
      count1 += 1;
    }
  }

  for mut text in query.iter_mut() {
    // text.sections[0].value = count.to_string();
    text.sections[0].value = format!("lod0: {} lod1: {}", count0, count1);
  }
}

fn update_player_pos(
  mut query: Query<&mut Text, With<PlayerPosText>>,
  char_query: Query<(&Transform, &Character)>,
) {

  let mut pos = Vec3::ZERO;
  for (transform, char) in char_query.iter() {
    pos = transform.translation;
  }

  for mut text in query.iter_mut() {
    // text.sections[0].value = count.to_string();
    text.sections[0].value = format!("Pos: {:?}", pos);
  }
}



#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct MeshCountText;

#[derive(Component)]
struct PlayerPosText;