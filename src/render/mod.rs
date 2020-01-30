use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use super::*;

pub mod imp;

pub struct Render<'a,C> where C: RenderTarget {
    pub c: &'a mut Canvas<C>,
}