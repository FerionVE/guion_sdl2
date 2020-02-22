extern crate guion_sdl2;

use context::Context;
use guion::{
    core::{
        ctx::{Context as GuionContext, Resolved, Widgets as GuionWidgets},
        env::Env,
        path::WPSlice,
        util::bounds::Bounds,
        widget::{link::Link, Widget},
        style::Color as GuionColor, render::link::RenderLink,
    },
    standard::ctx::StandardCtx,
    widgets::null::Null,
};
use guion_sdl2::render::Render;
use guion_sdl2::*;
use handler::Handler;
use sdl2::event::Event;
use sdl2::{keyboard::Keycode, video::Window, pixels::Color as SDLColor, rect::Rect};
use simple::{
    env::{SimpleEnv, SimpleID},
    stor::SimpleStor,
};
use render::imp::to_rect;
use guion_sdl2::style::color::Color;

fn main() {
    let sdl = sdl2::init().unwrap();

    let h: Handler<(), SimpleEnv> = Handler::new(());

    let mut c = Context::from_sdl2(sdl, h).unwrap();

    let g: Null<SimpleEnv> = Null::new(SimpleID::new(), Vec::new());
    let stor = SimpleStor::new(Box::new(g));
    let resolved = stor.widget(WPSlice { slice: &[] }).unwrap();

    //TODO Widget resolve impl

    let window = c
        .video
        .window("GUION_SDL2", 800, 600)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let mut r = Render::from_canvas(canvas);

    'running: loop {
        if let Some(event) = c.pump.wait_event_timeout(200) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
            r.c.set_draw_color(SDLColor::RGBA(0,0,0,0));
            let rect = r.c.viewport();
            eprintln!("Render");

            r.c.fill_rect(rect_0(&rect)).unwrap();

            RenderLink::simple(&mut r, (rect.width(),rect.height()), &mut c)
                .render_widget(c.link(resolved.clone()));

            r.c.present();
        }
    }
}

fn rect_0(r: &Rect) -> Rect {
    let mut r = r.clone();
    r.reposition((0,0));
    r
}