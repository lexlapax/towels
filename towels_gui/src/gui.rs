use nannou::prelude::*;
use std::string::ToString;

// /// All colors used in this application
// #[derive(Debug, Clone, Copy)]
// enum Color{
//     Honeydew,
//     SteelBlue,
// }

// impl ToString for Color {
//     fn to_string(&self) -> String {
//         format!("{:?}", self).to_lowercase()
//     }
// }

// /// Type alias for nannou color type
// type Rgb = Srgb<u8>;

// impl From<Color> for Rgb {
//     fn from(c: Color) -> Self {
//         named::from_str(&c.to_string()).unwrap()
//     }
// }

// /// A coordinate pair - the (0,0) default is the center of the frame
// #[derive(Debug, Default, Clone, Copy)]
// struct Point {
//     x: f32,
//     y: f32,
// }

// impl Point {
//     fn new(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }
// }

// /// Things that can be drawn to the screen
// trait Nannou {
//     fn display(&self, draw: &app::Draw);
//     fn update(&mut self);
// }


pub(crate) struct Model {
    bg_color: rgb::Srgb<u8>,
    x: f32,
    y: f32,
    radius: f32,
}

pub(crate) fn model(_app: &App) -> Model {
    Model {
        bg_color: HONEYDEW,
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }
}

pub(crate) fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.radius < 500.0 {
        model.radius += 1.0;
    }
}

pub(crate) fn view(_app: &App, model: &Model, frame: Frame){
    let draw = _app.draw();
    draw.background()
        .color(model.bg_color);
    draw.ellipse()
        .color(STEELBLUE)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(_app, &frame).unwrap();
}
