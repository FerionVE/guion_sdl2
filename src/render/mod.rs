use guion::core::util::bounds::Bounds;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

pub mod imp;

pub struct Render<'a,C> where C: RenderTarget {
    pub c: &'a mut Canvas<C>,
}