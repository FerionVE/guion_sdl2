extern crate guion_sdl2;

use crate::guion_sdl2::qwutils::ResultNonDebugUnwrap;
use guion::{
    ctx::{queue::StdEnqueueable, queue::StdOrder},
    widget::*,
    layout::Orientation,
    layout::*,
    widgets::{pane::Pane, button::Button, label::Label, pbar::ProgressBar, checkbox::CheckBox, splitpane::SplitPane, textbox::{state::Cursor, TextBox}, util::{state::AtomStateMut, caption::CaptionMut}},
    id::standard::StdID,
    aliases::WidgetRefMut, path::standard::SimplePath,
    constraint,const_std_id, validation::validated::Validated,
};
use guion_sdl2::*;
use simple::{
    env::{SimpleEnv},
    stor::SimpleStor, ctx::SimpleCtx, StandardPath, Simplion,
};
use link::Link;
use root::Widgets;

const_std_id!(RootPane ProgBar);

//minimal example using the simple module
fn main() {
    let mut simplion = Simplion::new();

    //special bounds for progressbar and checkbox
    let pb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis{min: 32, preferred: 64, max: Some(64), pressure: 1.0}};
    let cb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis::fixed(24)};

    let b_bounds = constraint!(~0-|64);
    let pb_bounds = constraint!(~0-|32~48);
    let cb_bounds = constraint!(~0-|24);

    //build a widget
    let g = Pane::new(
        RootPane(),
        Orientation::Vertical,
        (
            Label::new(StdID::new())
                .with_size(cb_bounds.clone())
                .with_text(Validated::new("Label".to_owned())),
            Pane::new(
                StdID::new(),
                Orientation::Horizontal,
                (
                    Button::new(StdID::new())
                        .with_size(b_bounds.clone())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Button::new(StdID::new())
                        .with_size(b_bounds.clone())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                ),
            ),
            ProgressBar::new(ProgBar(), Orientation::Horizontal)
                .with_value(0.5)
                .with_size(pb_bounds.clone()),
            CheckBox::new(StdID::new(), false)
                .with_text("CheckBox")
                .with_size(cb_bounds.clone()),
            SplitPane::new(
                StdID::new(), Orientation::Horizontal, 0.5,
                (
                    Button::new(StdID::new())
                        .with_size(b_bounds.clone())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Button::new(StdID::new())
                        .with_size(b_bounds.clone())
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
        let text = w.traitcast_mut::<dyn CaptionMut<SimpleEnv>>().unwrap();
        let i: u32 = text.caption().parse().unwrap();
        text.replace(&(i+1).to_string());
    }
    l.for_child(0).unwrap().mutate(button_mutate);

    fn update_pbar(s: &mut SimpleStor, c: &mut SimpleCtx) {
        /*let mut pbar = s.roots[0].0
            .child_mut(2).unwrap()
            .as_widget().unwrap_nodebug();*/
        let pbar_path = SimplePath::new(&[RootPane(),ProgBar()]);
        let mut pbar = s.widget_mut(pbar_path).unwrap();

        let pbar = pbar.traitcast_mut::<dyn AtomStateMut<SimpleEnv,f32>>().unwrap();
        pbar.set( (pbar.get(c)+0.1)%1.0, c );
    }
    l.enqueue(StdEnqueueable::MutateRoot{f: update_pbar}, StdOrder::PostCurrent, 0);
}
