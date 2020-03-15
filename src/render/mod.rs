use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use super::*;
use util::RenderSurface;

pub mod imp;
pub mod font;
pub mod util;

pub struct Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    pub c: Canvas<C>,
}

impl<C> Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    pub fn from_canvas(c: Canvas<C>) -> Self {
        Self{
            c
        }
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