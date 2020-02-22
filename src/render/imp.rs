use crate::style::color::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use guion::core::{style::cursor::StdCursor, render::widgets::RenderStdWidgets};
use super::*;
use style::{cursor::to_sdl_cursor, Style};

impl<E,C> GuionRender<E> for Render<C> where E: Env, E::Backend: GuionBackend<E,Renderer=Self>, C: RenderTarget {

}

impl<E,C> RenderStdWidgets<E> for Render<C> where
    E: Env + Sync,
    E::Backend: GuionBackend<E,Renderer=Self>,
    EStyle<E>: AsRefMut<Style>,
    ESColor<E>: Into<Color>,
    ESCursor<E>: Into<StdCursor>,
    E::Context: AsRefMut<Core<E>>,
    C: RenderTarget,
{
    #[inline]
    fn fill_rect(&mut self, b: &Bounds, c: ESColor<E>) {
        self.c.set_blend_mode(BlendMode::None);
        self.c.set_draw_color(c.into().v);
        self.c.fill_rect(to_rect(b)).expect("SDL Render Failure @ fill_rect");
    }
    #[inline]
    fn border_rect(&mut self, b: &Bounds, c: ESColor<E>, thickness: u32) {
        if thickness == 0 {return;}
        self.c.set_blend_mode(BlendMode::None);
        self.c.set_draw_color(c.into().v);
        for i in 1..thickness {
            if let Some(r) = to_rect(&b.step((i-1) as i32)) {
                self.c.draw_rect(r).expect("SDL Render Failure @ draw_rect");
            }
        }
    }
    #[inline]
    fn render_preprocessed_text(&mut self, b: &Bounds, text: &ESPPText<E>, style: &EStyle<E>, variant: &ESVariant<E>, c: &mut E::Context) {
        todo!()
    }
    #[inline]
    fn set_cursor(&mut self, b: &Bounds, cursor: ESCursor<E>) {
        SDLCursor::from_system(to_sdl_cursor(cursor)).unwrap().set() //Cursor::set
    }
    #[inline]
    fn draw_text_button(&mut self, b: &Bounds, pressed: bool, caption: &str, style: &EStyle<E>, variant: &ESVariant<E>) {
        todo!()
    }
    #[inline]
    fn draw_selected(&mut self, b: &Bounds, s: &EStyle<E>, variant: &ESVariant<E>) {
        todo!()
    }
}
#[inline]
pub fn to_rect(b: &Bounds) -> Option<Rect> {
    if b.size.w > 0 && b.size.h > 0 {
        Some(Rect::new(b.off.x,b.off.y,b.size.w,b.size.h))
    }else{
        None
    }
}