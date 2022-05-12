use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup_text)
      .add_system(update_text);
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


#[derive(Component)]
struct FpsText;