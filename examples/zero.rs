extern crate guion_sdl2;

use crate::guion_sdl2::qwutils::ResultNonDebugUnwrap;
use guion::{aliases::WidgetRefMut, const_std_id, constraint, ctx::{queue::StdEnqueueable, queue::StdOrder}, id::standard::StdID, layout::Orientation, layout::*, path::standard::SimplePath, validation::validated::Validated, widget::*, widgets::{area::Area, button::Button, checkbox::CheckBox, label::Label, pane::Pane, pbar::ProgressBar, splitpane::SplitPane, textbox::{state::Cursor, TextBox}, util::{state::AtomStateMut, caption::CaptionMut}}};
use guion_sdl2::{simple::immediate_test::{ImmediateLabel, ImmediateTextBox}};
use guion_sdl2::simple::{
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
    //let pb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis{min: 32, preferred: 64, max: Some(64), pressure: 1.0}};
    //let cb_bounds = Size{x: SizeAxis::empty(), y: SizeAxis::fixed(24)};

    let b_bounds = constraint!(~40-|64);
    let pb_bounds = constraint!(~0-|32~48);
    let cb_bounds = constraint!(~0-|24);

    //build a widget
    let g = Pane::new(
        RootPane(),
        Orientation::Vertical,
        (
            Label::new(StdID::new())
                .with_size(cb_bounds)
                .with_text(Validated::new("Label".to_owned())),
            Area::new(
                StdID::new(),
                Pane::new(
                    StdID::new(),
                    Orientation::Horizontal,
                    (
                        Button::new(StdID::new())
                            .with_size(b_bounds)
                            .with_text("0".to_owned())
                            .with_trigger(button_action),
                        Button::new(StdID::new())
                            .with_size(b_bounds)
                            .with_text("0".to_owned())
                            .with_trigger(button_action),
                    ),
                ),
            )
                .with_state((0,0)),
            ProgressBar::new(ProgBar(), Orientation::Horizontal)
                .with_value(0.5)
                .with_size(pb_bounds),
            CheckBox::new(StdID::new(), false)
                .with_text("CheckBox".to_owned())
                .with_size(cb_bounds),
            SplitPane::new(
                StdID::new(), Orientation::Horizontal, 0.5,
                (
                    Button::new(StdID::new())
                        .with_size(b_bounds)
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Button::new(StdID::new())
                        .with_size(b_bounds)
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                ),
            ),
            TextBox::new(StdID::new()),
            ImmediateLabel{text:"Immediate Label".to_owned(),id:StdID::new()} ,
            ImmediateTextBox{text:"Immediate TextBox".to_owned(),id:StdID::new()},
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
        let mut debug_type_names = vec![];
        w.debug_type_name(&mut debug_type_names);
        for i in debug_type_names {
            eprintln!("\t{}",i);
        }
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
