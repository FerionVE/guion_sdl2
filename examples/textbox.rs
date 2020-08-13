extern crate guion_sdl2;

use guion::{
    widgets::textbox::TextBox,
    id::standard::StdID, validation::{validated::Validated, Validation},
};
use guion_sdl2::*;
use simple::Simplion;

//minimal example using the simple module
fn main() {
    let mut simplion = Simplion::new();

    //build a widget
    let g = TextBox::new(StdID::new())
        .with_text(Validated::new("".to_owned()));


    //create a sdl window
    let window = simplion.ctx
        .video
        .window("TextBox", 820, 440)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    simplion.push_window(window, g);

    while simplion.do_events() {}
}
