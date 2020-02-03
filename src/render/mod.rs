use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use super::*;

pub mod imp;

pub struct Render<C> where C: RenderTarget {
    pub c: Canvas<C>,
}