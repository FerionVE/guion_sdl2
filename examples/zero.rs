extern crate guion_sdl2;

use guion::{
    core::{
        ctx::{Context as GuionContext, Widgets as GuionWidgets},
        widget::{link::Link, Widget, WidgetMut},
        style::{StdVerb, Color as GuionColor}, render::link::RenderLink, lazout::Orientation, util::bounds::Bounds,
        lazout::Size,
    },
    standard::handler::StdHandler,
    widgets::{pain::{toggle::TOwned, Pane}, beton::Beton},
};
use guion_sdl2::render::Render;
use guion_sdl2::*;
use crate::core::{post_render_events, pre_render_events, render_and_events};
use sdl2::event::Event;
use sdl2::{keyboard::Keycode, pixels::Color as SDLColor, rect::Rect};
use simple::{
    env::{SimpleEnv, SimpleID},
    stor::SimpleStor, ctx::SimpleCtx, path::SimplePath,
};
use event::cast::parse_event;
use std::any::Any;

//minimal example using the simple module
fn main() {
    //initialze sdl
    let sdl = sdl2::init().unwrap();
    let ttf = sdl2::ttf::init().unwrap();

    //create a SimpleCtx context
    let mut c = SimpleCtx::from_sdl2(sdl,ttf).unwrap();

    //build a widget
    let g: Pane<_,SimpleEnv> = Pane::new(
        SimpleID::new(),
        vec![
            //Null::<SimpleEnv>::new(SimpleID::new(), vec![]).erase_move(),
            Pane::new(
                SimpleID::new(),
                vec![
                    Beton::new(SimpleID::new(), Size::empty())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                    Beton::new(SimpleID::new(), Size::empty())
                        .with_text("0".to_owned())
                        .with_trigger(button_action),
                ],
                Orientation::Horizontal,
            ),
            //Null::<SimpleEnv>::new(SimpleID::new(), vec![]).erase_move(),
        ],
        Orientation::Vertical,
    );

    let root_path = SimplePath::new(&[],g.id());
    //build the widget tree root
    let mut stor = SimpleStor::new(Box::new(g));

    //TODO Widget resolve impl

    //create a sdl window
    let window = c
        .video
        .window("GUION_SDL2", 800, 600)
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
            let parsed = parse_event::<SimpleEnv>(&event, bounds); //TODO window size

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

fn rect_0(r: &Rect) -> Rect {
    let mut r = r.clone();
    r.reposition((0,0));
    r
}

fn button_action(mut l: Link<SimpleEnv>) {
    fn button_mutate(w: &mut dyn WidgetMut<SimpleEnv>) {
        w.debug_type_name();
        //TODO impl "intelligent" downcast (ref elision, etc.)
        //let w = w.w_downcast_mut::<&mut dyn WidgetMut<SimpleEnv>>().unwrap();
        let w = w.downcast_mut::<Beton<SimpleEnv,String>>().unwrap();
        let i: u32 = w.text.parse().unwrap();
        w.text = (i+1).to_string();
    }

    l.mutate(button_mutate, true);
}