use guion::core::ctx::Env;
use guion::core::render::Render as GuionRender;
use super::*;

impl<'a,E,C> GuionRender<E> for Render<'a,C> where E: Env<Renderer=Self>, C: RenderTarget {

}