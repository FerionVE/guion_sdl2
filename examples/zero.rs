extern crate guion_sdl2;

use crate::guion_sdl2::qwutils::ResultNonDebugUnwrap;
use guion::{
    ctx::{Context as GuionContext, queue::StdEnqueueable, queue::StdOrder},
    widget::*,
    widget::root::*,
    style::variant::StdVerb, render::link::RenderLink, layout::Orientation, util::bounds::Bounds,
    layout::*,
    widgets::{pane::Pane, button::Button, label::Label, pbar::ProgressBar, checkbox::CheckBox, splitpane::SplitPane, textbox::{state::Cursor, TextBox}},
    id::standard::StdID,
    aliases::WidgetRefMut,
};
use guion_sdl2::render::Render;
use guion_sdl2::*;
use crate::core::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use simple::{
    env::{SimpleEnv},
    stor::SimpleStor, ctx::SimpleCtx, StandardPath, Simplion,
};
use event::cast::parse_event;
use link::Link;

//minimal example using the simple module
fn main() {
    let mut simplion = Simplion::new();

    //special bounds for progressbar and checkbox
    let pb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis{min: 32, preferred: 64, max: Some(64), pressure: 1.0}};
    let cb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis{min: 32, preferred: 32, max: Some(32), pressure: 1.0}};

    //build a widget
    let g = Pane::new(
        StdID::new(),
        Orientation::Vertical,
        (
            Label::new(StdID::new())
                .with_text("Label".to_owned()),
            Pane::new(
                StdID::new(),
                Orientation::Horizontal,
                (
                    Button::new(StdID::new())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Button::new(StdID::new())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                ),
            ),
            ProgressBar::new(StdID::new(), Orientation::Horizontal)
                .with_value(0.5)
                .with_size(pb_bounds),
            CheckBox::new(StdID::new(), false)
                .with_text("CheckBox")
                .with_size(cb_bounds),
            SplitPane::new(
                StdID::new(), Orientation::Horizontal, 0.5,
                (
                    Button::new(StdID::new())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Button::new(StdID::new())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                ),
            ),
            TextBox::new(StdID::new()),
            ("Immediate Label".to_owned(),StdID::new()),
            ("Immediate TextBox".to_owned(),StdID::new(),(0,0),Cursor::default(),None),
        ),
    );


    //create a sdl window
    let window = simplion.ctx
        .video
        .window("GUION_SDL2", 820, 440)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    simplion.push_window(window, g);

    while simplion.do_events() {}
}

fn button_action(mut l: Link<SimpleEnv>) {
    fn button_mutate(mut w: WidgetRefMut<SimpleEnv>, _: &mut SimpleCtx, _: StandardPath) {
        w.debug_type_name();
        let w = w.downcast_mut::<Button<SimpleEnv,String>>().unwrap();
        let i: u32 = w.text.parse().unwrap();
        w.text = (i+1).to_string();
    }
    l.mutate(button_mutate);

    fn update_pbar(s: &mut SimpleStor, _: &mut SimpleCtx) {
        let mut pbar = s.roots[0].
            child_mut(2).unwrap()
            .as_widget().unwrap_nodebug();
        let pbar = pbar.downcast_mut::<ProgressBar<SimpleEnv>>().unwrap();
        pbar.value = (pbar.value+0.1)%1.0;
    }
    l.enqueue(StdEnqueueable::MutateRoot{f: update_pbar}, StdOrder::PostCurrent, 0);
}
