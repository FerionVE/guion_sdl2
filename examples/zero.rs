extern crate guion_sdl2;

use crate::guion_sdl2::qwutils::ResultNonDebugUnwrap;
use guion::{
    core::{
        ctx::{Context as GuionContext, Widgets as GuionWidgets, aliases::WidgetRefMut, StdEnqueueable},
        widget::*,
        style::StdVerb, render::link::RenderLink, layout::Orientation, util::bounds::Bounds,
        layout::Size, layout::SizeAxis,
    },
    widgets::{pane::Pane, button::Button, null::Null, label::Label, pbar::ProgressBar, checkbox::CheckBox},
};
use guion_sdl2::render::Render;
use guion_sdl2::*;
use crate::core::render_and_events;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use simple::{
    env::{SimpleEnv, SimpleID},
    stor::SimpleStor, ctx::SimpleCtx, path::SimplePath,
};
use event::cast::parse_event;
use link::Link;

//minimal example using the simple module
fn main() {
    //initialze sdl
    let sdl = sdl2::init().unwrap();
    let ttf = sdl2::ttf::init().unwrap();

    //create a SimpleCtx context
    let mut c = SimpleCtx::from_sdl2(sdl,ttf).unwrap();

    let pbbounds = Size{x: SizeAxis::empty(), y: SizeAxis{min: 32, preferred: 64, max: Some(64), pressure: 1.0}};

    //build a widget
    let g = Pane::new(
        SimpleID::new(),
        (
            Label::new(SimpleID::new())
                .with_text("Label".to_owned()),
            Pane::new(
                SimpleID::new(),
                (
                    Button::new(SimpleID::new())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Button::new(SimpleID::new())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                ),
                Orientation::Horizontal,
            ),
            ProgressBar::new(SimpleID::new(), Orientation::Horizontal)
                .with_value(0.5)
                .with_size(pbbounds),
            CheckBox::new(SimpleID::new(), false)
                .with_text("CheckBox"),
            Null::new(SimpleID::new()),
        ),
        Orientation::Vertical,
    );

    let root_path = SimplePath::new(&[],g.id());
    //build the widget tree root
    let mut stor = SimpleStor::new(Box::new(g));

    //create a sdl window
    let window = c
        .video
        .window("GUION_SDL2", 840, 480)
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
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

            //let sdl render it
            r.present();
        }
    }
}

fn button_action(mut l: Link<SimpleEnv>) {
    fn button_mutate(mut w: WidgetRefMut<SimpleEnv>, _: &mut SimpleCtx, _: SimplePath) {
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
