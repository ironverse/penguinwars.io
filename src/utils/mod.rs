use bevy::math::{Vec3, Quat};


pub struct Math;

impl Math {
  pub fn look_at_to_rotation_quat(look_at: Vec3) -> Quat {
    let rot = Math::look_at_to_rotation(look_at);
    // Quat::from_rotation_ypr(rot.y, rot.x, 0.0)
    Quat::from_rotation_y(rot.y) * Quat::from_rotation_x(rot.x)
  }

  pub fn look_at_to_rotation(look_at: Vec3) -> Vec3 {
    let tmp_look_at = look_at.normalize();
    let mut rad_x = tmp_look_at.y;
    if rad_x.is_nan() {
      rad_x = 0.0;
    }

    let mut rad_y = tmp_look_at.x / tmp_look_at.z;
    if rad_y.is_nan() {
      rad_y = 0.0;
    }

    let mut y_rot = rad_y.atan();
    if tmp_look_at.z > 0.0 {
      let half_pi = std::f32::consts::PI * 0.5;
      y_rot = -((half_pi) + (half_pi - y_rot));
    }

    Vec3::new(rad_x.asin(), y_rot, 0.0)
  }

  pub fn rot_to_look_at(rot: Vec3) -> Vec3 {
    let yaw = rot.y - std::f32::consts::PI * 0.5;

    let len = rot.x.cos();
    return Vec3::new(yaw.cos() * len, rot.x.sin(), -yaw.sin() * len).normalize();
  }
}