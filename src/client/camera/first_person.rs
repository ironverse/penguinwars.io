use bevy::prelude::*;

use super::{Anchor, CameraSettings};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(rotate)
      .add_system(movement);
  }
}

fn rotate(
  anchors: Query<&Anchor>,
  mut cam: Query<(&mut Transform, &mut CameraSettings)>
) {
  let mut target = Vec3::ZERO;
  for a in anchors.iter() {
    target = a.pos.clone();
  }

  for (mut trans, mut settings) in cam.iter_mut() {
    let yaw_radians = settings.yaw.to_radians();
    let pitch_radians = settings.pitch.to_radians();

    trans.rotation =
      Quat::from_axis_angle(Vec3::Y, yaw_radians) * Quat::from_axis_angle(-Vec3::X, pitch_radians);

    trans.translation = target.clone();
    // let reverse_lookat = Math::rot_to_look_at(Vec3::new(pitch_radians, yaw_radians, 0.0)) ;
    // let cam_pos = target + (reverse_lookat * 5.0);
    
    // let new = Transform::from_xyz(cam_pos[0], cam_pos[1], cam_pos[2])
    //   .looking_at(target, Vec3::Y);

    // trans.translation = new.translation;
    // trans.rotation = new.rotation;
    // trans.scale = new.scale;
  }
}

fn movement(
  mut anchors: Query<(&Transform, &mut Anchor)>
) {
  for (trans, mut anchor) in anchors.iter_mut() {
    anchor.pos = trans.translation.clone();
  }
}