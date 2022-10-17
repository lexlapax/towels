pub(crate) use crate::gui::*;
mod gui;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

