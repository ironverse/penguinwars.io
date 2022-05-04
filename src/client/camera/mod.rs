use bevy::input::mouse::{MouseMotion, MouseButtonInput};
use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PointerState::default())
      .add_startup_system(add_cam)
      .add_startup_system(add_light);

    app
      .add_system(input)
      .add_system(rotate)
      .add_system(mouse_motion_system)
      ;
  }
}

fn add_cam(mut commands: Commands) {
  commands
    .spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(0.0, 0.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    })
    .insert(CameraSettings::default());
}

fn add_light(mut commands: Commands) {
  commands.spawn_bundle(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
}

/* 
  TODO
    Only focus on the camera transform
    Then adjust accordingly

    v1
      Modes
        First person
          Pitch
          Yaw
        Third person
          Pitch
          Yaw

    v2
      Separate the movement from the camera control
*/
fn input(
  key_input: Res<Input<KeyCode>>,
) {

}

fn rotate(
  mut cam: Query<(&mut Transform, &mut CameraSettings), Changed<CameraSettings>>
) {
  for (mut trans, mut settings) in cam.iter_mut() {
    let yaw_radians = settings.yaw.to_radians();
    let pitch_radians = settings.pitch.to_radians();

    trans.rotation =
      Quat::from_axis_angle(Vec3::Y, yaw_radians) * Quat::from_axis_angle(-Vec3::X, pitch_radians);
  }
}

/* Setting Mouse settings, have to change name later */
fn mouse_motion_system(
  time: Res<Time>,
  mut state: ResMut<PointerState>,
  mut mouse_motion_events: EventReader<MouseMotion>,
  mut ev_mousebtn: EventReader<MouseButtonInput>,
  mut ev_cursor: EventReader<CursorMoved>,
  // fs_res: Res<WasmMouseTracker>,

  mut cam_settings: Query<&mut CameraSettings>
) {
  let mut delta: Vec2 = Vec2::ZERO;
  for event in mouse_motion_events.iter() {
    delta += event.delta;
  }

  // if fs_res.fullscreen {
  //   delta.x += fs_res.dx;
  //   delta.y += fs_res.dy
  // }

  for ev_mouse in ev_mousebtn.iter() {
    if ev_mouse.state.is_pressed() && ev_mouse.button == MouseButton::Left {
      state.dragged = true;
    }

    if !ev_mouse.state.is_pressed() && ev_mouse.button == MouseButton::Left {
      state.dragged = false;
    }
  }

  for ev in ev_cursor.iter() {
    if state.last_cursor_pos.length_squared() < 0.1 || !state.dragged {
      state.last_cursor_pos = ev.position;
      return;
    }
    delta = ev.position - state.last_cursor_pos;
    delta.y *= -1.0;
    state.last_cursor_pos = ev.position;
  }

  for (mut settings) in cam_settings.iter_mut() {
    settings.pitch -= delta.y * settings.pitch_speed * time.delta_seconds();
    settings.yaw += delta.x * settings.yaw_speed * time.delta_seconds();
    
    settings.pitch = settings.pitch.clamp(-89.9, 89.9);

    info!("settings.yaw {:?}", settings.yaw);
  }
}


#[derive(Component)]
pub struct CameraSettings {
  pub pitch: f32,
  pub yaw: f32,
  pub pitch_speed: f32,
  pub yaw_speed: f32,
}

impl Default for CameraSettings {
  fn default() -> Self {
    Self {
      pitch: 0.0,
      yaw: 180.0,
      pitch_speed: 10.0,
      yaw_speed: 10.0
    }
  }
}

#[derive(Component)]
struct PointerState {
  dragged: bool,
  last_cursor_pos: Vec2,
}

impl Default for PointerState {
  fn default() -> Self {
    Self {
      dragged: false,
      last_cursor_pos: Vec2::ZERO,
    }
  }
}
















use bevy::input::keyboard::KeyCode;

#[derive(Debug)]
pub struct InputMap {
  pub forward: KeyCode,
  pub backward: KeyCode,
  pub left: KeyCode,
  pub right: KeyCode,
  pub jump: KeyCode,
  pub run: KeyCode,
  pub crouch: KeyCode,
  pub invert_y: bool,
  pub fly: KeyCode,
  pub fly_up: KeyCode,
  pub fly_down: KeyCode,
}

impl Default for InputMap {
  fn default() -> Self {
    Self {
      forward: KeyCode::W,
      backward: KeyCode::S,
      left: KeyCode::A,
      right: KeyCode::D,
      jump: KeyCode::Space,
      run: KeyCode::LShift,
      crouch: KeyCode::LControl,
      invert_y: false,
      fly: KeyCode::F,
      fly_up: KeyCode::E,
      fly_down: KeyCode::Q,
    }
  }
}
