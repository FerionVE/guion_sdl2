pub mod env;
pub mod stor;
//pub mod path;
pub mod valid;
//pub mod style;
pub mod ctx;
use super::*;
use guion::{id::standard::StdID, path::standard::SimplePath, widget::WidgetMut, ctx::queue::StdOrder, render::link::RenderLink, style::variant::StdVerb};
use env::SimpleEnv;
use ctx::SimpleCtx;
use stor::SimpleStor;
use render::Render;
use sdl2::{render::Canvas, video::Window};
use SDLEvent;
use event::cast::{event_dest_window, parse_event};
pub mod immediate_test;
use guion::{event::{filter::StdFilter, compound::EventCompound}};

pub type StandardPath = SimplePath<SimpleEnv,StdID>;

pub struct Simplion {
    pub stor: SimpleStor,
    pub renderer: Render,
    pub ctx: SimpleCtx,
}

impl Simplion {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        Self{
            stor: SimpleStor::new(),
            renderer: Render::new(),
            ctx: SimpleCtx::from_sdl2(sdl).unwrap(),
        }
    }
    pub fn push_canvas<W>(&mut self, x: Canvas<Window>, w: W) where W: WidgetMut<'static,SimpleEnv> {
        assert_eq!(self.stor.roots.len(),self.renderer.windows.len());
        self.stor.roots.push((w.boxed(),x.window().size().into()));
        self.renderer.windows.push(x);
    }
    pub fn push_window<W>(&mut self, x: Window, w: W) where W: WidgetMut<'static,SimpleEnv> {
        self.push_canvas(x.into_canvas().build().unwrap(),w)
    }
    pub fn do_events(&mut self) -> bool {
        if let Some(event) = self.ctx.pump.wait_event_timeout(200) {
            process_events::<SimpleEnv>(&mut self.stor, &mut self.ctx, StdOrder::PreEvents);

            let mut feed = Some(event);
            //process all pending events
            while let Some(event) = feed {
                match event {
                    SDLEvent::Quit { .. } => return false,
                    _ => {}
                }

                println!("{:?}",event);

                let mut witer = 0..self.stor.roots.len();

                if let Some(id) = event_dest_window(&event) {
                    if let Some(idx) = self.renderer.window_by_id(id) {
                        witer = idx..(idx+1);
                    }
                }

                process_events::<SimpleEnv>(&mut self.stor, &mut self.ctx, StdOrder::PreEvent);

                for widx in witer {
                    self.renderer.current = widx;
                    let rect = self.renderer.windows[widx].viewport();
                    let bounds = (rect.width(),rect.height());
                    let bbounds = &Bounds::from_xywh(0,0,bounds.0,bounds.1);
                    let path = self.stor.path_for_root(widx);

                    let parsed = parse_event::<SimpleEnv>(&event, bounds);

                    self.stor.roots[widx].1 = bounds.into();

                    let e = EventCompound(
                        parsed.event,
                        Bounds::default(),
                        parsed.ts as u64,
                        StdFilter{
                            filter_path: path,
                            filter_bounds: true,
                        },
                        true,
                    );
                    
                    let mut link = self.ctx.link(self.stor.resolved());
                    link._event_root(&e);
                }

                process_events::<SimpleEnv>(&mut self.stor, &mut self.ctx, StdOrder::PostCurrent);
                process_events::<SimpleEnv>(&mut self.stor, &mut self.ctx, StdOrder::PostEvent);

                feed = self.ctx.pump.poll_event();
            }

            process_events::<SimpleEnv>(&mut self.stor, &mut self.ctx, StdOrder::PostEvents);

            eprintln!("Render");

            for widx in 0..self.stor.roots.len() { //TODO move render single windows to stor root
                self.renderer.current = widx;
                let rect = self.renderer.windows[widx].viewport();
                let bounds = (rect.width(),rect.height());
                let path = self.stor.path_for_root(widx);

                //build the RenderLink
                let mut rl = RenderLink::simple(&mut self.renderer, bounds, &mut self.ctx);
                //fill background
                rl.with(&[StdVerb::ObjBackground])
                    .fill_rect();
                //process queued and render
                render_and_events::<SimpleEnv>(&mut rl, path, &mut self.stor, &mut self.ctx);

                self.renderer.update_cursor();

                //let sdl render it
                self.renderer.windows[widx].present();
            }
        }
        true
    }
}
