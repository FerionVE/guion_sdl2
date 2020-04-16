use sdl2::render::Canvas;
use sdl2::{mouse::SystemCursor, render::RenderTarget};
use super::*;
use util::RenderSurface;

pub mod imp;
pub mod font;
pub mod util;

pub struct Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    pub c: Canvas<C>,
    pub cursor: SystemCursor,
    pub set_cursor: SystemCursor,
    pub curse: Option<SDLCursor>,
}

impl<C> Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    pub fn from_canvas(c: Canvas<C>) -> Self {
        Self{
            c,
            cursor: SystemCursor::Arrow,
            set_cursor: SystemCursor::Arrow,
            curse: None,
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
}

impl<C> Deref for Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    type Target = Canvas<C>;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
impl<C> DerefMut for Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.c
    }
}
impl<C> AsRefMut<Canvas<C>> for Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    fn as_ref(&self) -> &Canvas<C> {
        &self.c
    }
    fn as_mut(&mut self) -> &mut Canvas<C> {
        &mut self.c
    }
}
impl<C> AsRefMut<Self> for Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    fn as_ref(&self) -> &Self {
        self
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}