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
};
use guion_sdl2::*;
use simple::{
    env::{SimpleEnv},
    stor::SimpleStor, ctx::SimpleCtx, StandardPath, Simplion,
};
use link::Link;
use root::Widgets;

guion::const_std_id!(RootPane ProgBar);

/// WIP macro
macro_rules! sizion {
    (# $min:literal ~ $pref:literal - $max:tt @ $p:literal | $($m:tt)*) => {
        Size{
            x: sizion!(#$min ~ $pref - $max @ $p),
            y: sizion!($($m)*),
        }
    };
    (# $min:literal ~ $pref:literal - None @ $p:literal) => {
        SizeAxis{min:$min,preferred:$pref,max:None,pressure:$p}
    };
    (# $min:literal ~ $pref:literal - $max:literal @ $p:literal) => {
        SizeAxis{min:$min,preferred:$pref,max:Some($max),pressure:$p}
    };
    (# $min:literal ~ $pref:literal - $max:tt $($m:tt)*) => {
        sizion!(#$min ~ $pref - $max @ 1.0 $($m)*)
    };
    ($min:literal ~ $pref:literal - $max:literal $($m:tt)*) => {
        sizion!(#$min ~ $pref - $max $($m)*)
    };
    ($min:literal ~ $pref:literal - $($m:tt)*) => {
        sizion!(#$min ~ $pref - $pref $($m)*)
    };
    ($min:literal ~ $pref:literal $($m:tt)*) => {
        sizion!(#$min ~ $pref - None $($m)*)
    };
    (~ $pref:literal $($m:tt)*) => {
        sizion!($pref ~ $pref $($m)*)
    };
    ($($m:tt)*) => {
        sizion!(0 ~ $($m)*)
    };
}

//minimal example using the simple module
fn main() {
    let mut simplion = Simplion::new();

    //special bounds for progressbar and checkbox
    let pb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis{min: 32, preferred: 64, max: Some(64), pressure: 1.0}};
    let cb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis::fixed(24)};

    let pb_bounds = sizion!(0|32~64-);
    let cb_bounds = sizion!(0|~24-);

    //build a widget
    let g = Pane::new(
        RootPane(),
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
            ProgressBar::new(ProgBar(), Orientation::Horizontal)
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
        let text = w.traitcast_mut::<dyn CaptionMut>().unwrap();
        let i: u32 = text.caption().parse().unwrap();
        text.replace(&(i+1).to_string());
    }
    l.mutate(button_mutate);

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
