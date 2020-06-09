use sdl2::render::Canvas;
use sdl2::{mouse::SystemCursor, render::{Texture, RenderTarget, WindowCanvas}};
use super::*;
use util::RenderSurface;
use rusttype::Font;
use font::load_font;

pub mod imp;
pub mod font;
pub mod util;

pub struct Render {
    pub windows: Vec<WindowCanvas>,
    pub current: usize,
    pub cursor: SystemCursor,
    pub set_cursor: SystemCursor,
    pub curse: Option<SDLCursor>,
    pub cache: rusttype::gpu_cache::Cache<'static>,
    pub cache_tex: Option<Texture<'static>>,
}

impl Render {
    pub fn new() -> Self {
        Self{
            windows: Vec::new(),
            current: 0,
            cursor: SystemCursor::Arrow,
            set_cursor: SystemCursor::Arrow,
            curse: None,
            cache: rusttype::gpu_cache::Cache::builder().build(),
            cache_tex: None,
        }
    }

    pub fn update_cursor(&mut self) {
        //eprintln!("{:?} VS {:?}",self.set_cursor,self.cursor);
        if self.cursor != self.set_cursor {
            //eprintln!("CURSOR SET {:?}",self.cursor);
            self.curse = Some(SDLCursor::from_system(self.cursor).unwrap());
            self.curse.as_ref().unwrap().set();
            
        }
        self.set_cursor = self.cursor;
            self.cursor = SystemCursor::Arrow;
    }

    pub fn window_by_id(&self, window_id: u32) -> Option<usize> {
        self.windows.iter()
            .enumerate()
            .filter(|(_,w)| w.window().id() == window_id )
            .map(|(i,_)| i )
            .next()
    }
}

impl AsRefMut<Self> for Render {
    fn as_ref(&self) -> &Self {
        self
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}