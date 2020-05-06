extern crate guion_sdl2;

use crate::guion_sdl2::qwutils::ResultNonDebugUnwrap;
use guion::{
    ctx::{Context as GuionContext, queue::StdEnqueueable},
    widget::*,
    widget::root::*,
    style::variant::StdVerb, render::link::RenderLink, layout::Orientation, util::bounds::Bounds,
    layout::*,
    widgets::{pane::Pane, button::Button, label::Label, pbar::ProgressBar, checkbox::CheckBox, splitpane::SplitPane, textbox::TextBox},
    id::standard::StdID,
    aliases::WidgetRefMut,
};
use guion_sdl2::render::Render;
use guion_sdl2::*;
use crate::core::render_and_events;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use simple::{
    env::{SimpleEnv},
    stor::SimpleStor, ctx::SimpleCtx, StandardPath,
};
use event::cast::parse_event;
use link::Link;

//minimal example using the simple module
fn main() {
    //initialze sdl
    let sdl = sdl2::init().unwrap();

    //create a SimpleCtx context
    let mut c = SimpleCtx::from_sdl2(sdl).unwrap();

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
        ),
    );

    let root_path = StandardPath::new(&[]);
    //build the widget tree root
    let mut stor = SimpleStor::new(Box::new(g));

    //create a sdl window
    let window = c
        .video
        .window("GUION_SDL2", 820, 440)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    //retrieve the canvas and build the renderer
    let canvas = window.into_canvas().build().unwrap();
    let mut r = Render::from_canvas(canvas);

    //main loop
    'running: loop {
        //wait/poll events
        if let Some(event) = c.pump.wait_event_timeout(200) {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }

            let rect = r.c.viewport();
            let bounds = (rect.width(),rect.height());
            let bbounds = &Bounds::from_xywh(0,0,bounds.0,bounds.1);

            println!("{:?}",event);

            //parse event
            let parsed = parse_event::<SimpleEnv>(&event, bounds);

            //feed event into context
            c.link(stor.widget(root_path.clone()).unwrap())
                ._event_root(
                    (parsed.event,bbounds,parsed.ts as u64)
                );

            eprintln!("Render");

            //build the RenderLink
            let mut rl = RenderLink::simple(&mut r, bounds, &mut c);
            //fill background
            rl.with(&[StdVerb::ObjBackground])
                .fill_rect();
            //process queued and render
            render_and_events::<SimpleEnv>(&mut rl, root_path.clone(), &mut stor, &mut c);

            r.update_cursor();

            //let sdl render it
            r.present();
        }
    }
}

fn button_action(mut l: Link<SimpleEnv>) {
    fn button_mutate(mut w: WidgetRefMut<SimpleEnv>, _: &mut SimpleCtx, _: StandardPath) {
        w.debug_type_name();
        let w = w.downcast_mut::<Button<SimpleEnv,String>>().unwrap();
        let i: u32 = w.text.parse().unwrap();
        w.text = (i+1).to_string();
    }
    l.mutate(button_mutate, true);

    fn update_pbar(s: &mut SimpleStor, _: &mut SimpleCtx) {
        let mut pbar = s.root.
            child_mut(2).unwrap()
            .as_widget().unwrap_nodebug();
        let pbar = pbar.downcast_mut::<ProgressBar<SimpleEnv>>().unwrap();
        pbar.value = (pbar.value+0.1)%1.0;
    }
    l.enqueue(StdEnqueueable::MutateRoot{f: update_pbar});
}
