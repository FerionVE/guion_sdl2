extern crate guion_sdl2;

use guion::{
    core::{
        ctx::{Context as GuionContext, Widgets as GuionWidgets},
        widget::{link::Link, Widget},
        style::Color as GuionColor, render::link::RenderLink, lazout::Orientation, util::bounds::Bounds,
    },
    standard::handler::StdHandler,
    widgets::{pain::Pane, null::Null},
};
use guion_sdl2::render::Render;
use guion_sdl2::*;
use sdl2::event::Event;
use sdl2::{keyboard::Keycode, pixels::Color as SDLColor, rect::Rect};
use simple::{
    env::{SimpleEnv, SimpleID},
    stor::SimpleStor, ctx::SimpleCtx, path::SimplePath,
};
use event::cast::parse_event;

//minimal example using the simple module
fn main() {
    //initialze sdl
    let sdl = sdl2::init().unwrap();

    //create a SimpleCtx context
    let mut c = SimpleCtx::from_sdl2(sdl).unwrap();

    //build a widget
    let g = Pane::new(
        SimpleID::new(),
        vec![
            Null::new(SimpleID::new(), vec![]),
            Null::new(SimpleID::new(), vec![]),
        ],
        Orientation::Vertical,
    );

    let root_path = SimplePath::new(&[],g.id());
    //build the widget tree root
    let stor = SimpleStor::new(Box::new(g));
    //reference to the root widget
    let resolved = stor.widget(root_path).unwrap();

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

            let parsed = parse_event::<SimpleEnv>(&event, (0f32,0f32)); //TODO window size

            c.link(resolved.clone())._event_root(
                (parsed.event,bbounds,parsed.ts as u64)
            );

            r.c.set_draw_color(SDLColor::RGBA(0,0,0,0));
            
            eprintln!("Render");

            //black the background
            eprintln!("Render");
            r.set_draw_color(SDLColor::RGBA(0,0,0,0));
            let rect = r.c.viewport();
            r.fill_rect(rect_0(&rect)).unwrap();

            //build the RenderLink and call it on the root widget
            RenderLink::simple(&mut r, bounds, &mut c)
                .render_widget(c.link(resolved.clone()));

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