use std::{borrow::Cow, default};

use bevy_egui::egui::{
  self,
  style::{Spacing, WidgetVisuals, Widgets, Margin},
  vec2, Color32,
  FontFamily::{Monospace, Proportional},
  Stroke, TextStyle, Vec2, Visuals, Rounding, FontDefinitions, FontData, FontTweak,
};

const SUE_ELLEN_FRANCISCO_FONT: Cow<[u8]> = Cow::Borrowed(include_bytes!(
  "../../../../assets/fonts/Sue-Ellen-Francisco.ttf"
));
const ARIMO_REGULAR_FONT: Cow<[u8]> =
  Cow::Borrowed(include_bytes!("../../../../assets/fonts/Arimo-Regular.ttf"));

const FIRASANS_BOLD: Cow<[u8]> =
  Cow::Borrowed(include_bytes!("../../../../assets/fonts/FiraSans-Bold.ttf"));

const FIRAMONO_MEDIUM: Cow<[u8]> =
  Cow::Borrowed(include_bytes!("../../../../assets/fonts/FiraMono-Medium.ttf"));

const REFERENCE_SIZE: [f32; 2] = [1920.0, 1080.0];
const FONT_SCALE: f32 = 1.20;

pub fn setup_style(ctx: &egui::Context) -> Scaler {
  let s = Scaler::new(ctx.available_rect().size().into(), REFERENCE_SIZE);
  let size = ctx.available_rect().size();

  let mut fd = FontDefinitions::default();
  // let mut fd = ctx.fonts().definitions().clone();
  fd.font_data
    .entry("Monospace".into())
    .or_insert(FontData {
      font: SUE_ELLEN_FRANCISCO_FONT,
      index: 0,
      tweak: FontTweak::default(),
    });
  fd.font_data
    .entry("Proportional".into())
    .or_insert(FontData {
      font: ARIMO_REGULAR_FONT,
      index: 0,
      tweak: FontTweak::default(),
    });

  fd.font_data
    .entry("FiraSans_Bold".into())
    .or_insert(FontData {
      font: FIRASANS_BOLD,
      index: 0,
      tweak: FontTweak::default(),
    });
  
  fd.font_data
    .entry("Medium".into())
    .or_insert(FontData {
      font: FIRAMONO_MEDIUM,
      index: 0,
      tweak: FontTweak::default(),
    });
  // fd.font_data
  //   .insert("Medium".into(),
  //     FontData::from_static(include_bytes!("../../../../assets/fonts/FiraMono-Medium.ttf"))
  //   );

  fd.families
    .insert(Monospace, vec![
      "Medium".into(),
    ]);

  fd.families
    .insert(Proportional, vec!["Proportional".into()]);
  // fd.families
  //   .insert(egui::FontFamily::Name("Medium".into()), vec!["Medium".into()]); // NOT WORKING
    
  // for fd in fd.font_data.iter() {
  //   println!("fd {:?}", fd.0);
  // }
  
  // for fam in fd.families.iter() {
  //   println!("fam {:?}", fam.0);
  // }

  // fd.families
  //   .insert(TextStyle::Heading, (Proportional, s.font(36.0)));
  // fd.families
  //   .insert(TextStyle::Button, (Proportional, s.font(28.0)));
  // fd.families
  //   .insert(TextStyle::Body, (Proportional, s.font(20.0)));
  // fd.families
  //   .insert(TextStyle::Small, (Proportional, s.font(16.0)));


  // fd.fonts_for_family
  //   .insert(Monospace, vec!["Monospace".into()]);
  // fd.fonts_for_family
  //   .insert(Proportional, vec!["Proportional".into()]);
  // fd.family_and_size
  //   .insert(TextStyle::Monospace, (Monospace, s.font(150.0)));
  // fd.family_and_size
  //   .insert(TextStyle::Heading, (Proportional, s.font(36.0)));
  // fd.family_and_size
  //   .insert(TextStyle::Button, (Proportional, s.font(28.0)));
  // fd.family_and_size
  //   .insert(TextStyle::Body, (Proportional, s.font(20.0)));
  // fd.family_and_size
  //   .insert(TextStyle::Small, (Proportional, s.font(16.0)));
  ctx.set_fonts(fd);

  ctx.set_style(egui::Style {
    spacing: Spacing {
      item_spacing: vec2(8.0, 3.0),
      window_margin: Margin {
        ..Default::default()
      },
      button_padding: vec2(15.0, 6.0),
      indent: 25.0,
      interact_size: vec2(40.0, 20.0),
      slider_width: 100.0,
      text_edit_width: 280.0,
      icon_width: 16.0,
      icon_spacing: 0.0,
      tooltip_width: 600.0,
      ..Default::default()
    },
    visuals: Visuals {
      widgets: Widgets {
        noninteractive: WidgetVisuals {
          bg_fill: Color32::TRANSPARENT, //Color32::from_gray(30), // window background
          bg_stroke: Stroke::new(1.0, Color32::from_gray(65)), // window outline
          fg_stroke: Stroke::new(1.0, Color32::WHITE), //Color32::from_gray(160)), // normal text color
          rounding: Rounding {
            ..Default::default()
          },
          expansion: 0.0,
        },
        inactive: WidgetVisuals {
          bg_fill: Color32::from_rgb(156, 59, 119), //Color32::from_gray(70),
          bg_stroke: Default::default(),
          fg_stroke: Stroke::new(1.0, Color32::from_gray(230)), // Should NOT look grayed out!
          rounding: Rounding {
            ..Default::default()
          },
          expansion: 0.0,
        },
        hovered: WidgetVisuals {
          bg_fill: Color32::from_rgb(172, 65, 131), //Color32::from_gray(80),
          bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
          fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
          rounding: Rounding {
            ..Default::default()
          },
          expansion: 1.0,
        },
        active: WidgetVisuals {
          bg_fill: Color32::from_rgb(189, 72, 144), //Color32::from_gray(90),
          bg_stroke: Stroke::new(1.0, Color32::WHITE),
          fg_stroke: Stroke::new(2.0, Color32::WHITE),
          rounding: Rounding {
            ..Default::default()
          },
          expansion: 2.0,
        },
        open: WidgetVisuals {
          bg_fill: Color32::from_rgb(189, 72, 144), //Color32::from_gray(90),
          bg_stroke: Stroke::new(1.0, Color32::WHITE),
          fg_stroke: Stroke::new(2.0, Color32::WHITE),
          rounding: Rounding {
            ..Default::default()
          },
          expansion: 2.0,
        },
      },
      ..Default::default()
    },
    ..Default::default()
  });

  return s;
}

pub struct Scaler {
  pub screen_size: [f32; 2],
  pub reference_size: [f32; 2],
}

impl Scaler {
  pub fn new(screen_size: [f32; 2], reference_size: [f32; 2]) -> Self {
    Scaler {
      screen_size: screen_size,
      reference_size: reference_size,
    }
  }
  pub fn xy(&self, widget_size: [f32; 2]) -> [f32; 2] {
    let x_ratio = self.screen_size[0] / self.reference_size[0];
    let y_ratio = self.screen_size[1] / self.reference_size[1];
    return [widget_size[0] * x_ratio, widget_size[1] * y_ratio];
  }
  pub fn x(&self, size: f32) -> f32 {
    let x_ratio = self.screen_size[0] / self.reference_size[0];
    return size * x_ratio;
  }
  pub fn y(&self, size: f32) -> f32 {
    let y_ratio = self.screen_size[1] / self.reference_size[1];
    return size * y_ratio;
  }
  pub fn font(&self, size: f32) -> f32 {
    return self.y(size * FONT_SCALE);
  }
}