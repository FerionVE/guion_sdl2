use crate::style::color::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use guion::{style::variant::standard::StdCursor, render::widgets::RenderStdWidgets};
use super::*;
use style::{cursor::to_sdl_cursor, Style, font::Glyphs};

impl<E> GuionRender<E> for Render where E: Env, ERenderer<E>: AsRefMut<Self> {

}

impl<E> RenderStdWidgets<E> for Render where
    E: Env + Sync,
    ERenderer<E>: AsRefMut<Self>,
    EStyle<E>: AsRefMut<Style>,
    ESGlyphs<E>: AsRefMut<Glyphs>,
    ESColor<E>: Into<Color>,
    ESCursor<E>: Into<StdCursor>,
    E::Context: AsRefMut<Core<E>>,
{
    #[inline]
    fn fill_rect(&mut self, b: &Bounds, c: ESColor<E>) {
        if let Some(b) = to_rect(b) {
            let r = &mut self.windows[self.current];
            r.set_blend_mode(BlendMode::None);
            r.set_draw_color(c.into().v);
            r.fill_rect(Some(b)).expect("SDL Render Failure @ fill_rect");
        }
    }
    #[inline]
    fn border_rect(&mut self, b: &Bounds, c: ESColor<E>, thickness: u32) {
        if thickness == 0 {return;}
        let x = &mut self.windows[self.current];
        x.set_blend_mode(BlendMode::None);
        x.set_draw_color(c.into().v);
        for i in 0..thickness {
            if let Some(r) = to_rect(&b.step(i as i32)) {
                x.draw_rect(r).expect("SDL Render Failure @ draw_rect");
            }
        }
    }
    /*#[inline]
    fn render_text(&mut self, b: &Bounds, text: &str, align: (f32,f32), style: &EStyle<E>, variant: &ESVariant<E>, ctx: &mut E::Context) {
        let (glyphs,bounds) = 
            glyphs_of_str(&ctx.as_ref().font,Scale::uniform(24.0), std::i32::MAX as u32, text);
        
        let b = b.inner_aligned_f((bounds.x,bounds.y),align);

        if b.not_empty() {
            //self.c.set_draw_color(SDLColor::RGBA(255, 0, 0, 255));
            //self.c.fill_rect(to_rect(&b)).expect("SDL Render Failure @ fill_rect");
            //self.c.set_blend_mode(BlendMode::Blend);
            let color = style.color(variant);
            self.render_glyphs(b, Offset::default(), color.into().v, glyphs.into_iter()).expect("TTOOF");
        }
    }*/
    #[inline]
    fn render_preprocessed_text(&mut self, b: &Bounds, text: &ESGlyphs<E>, inner_offset: Offset, style: &EStyle<E>, variant: &ESVariant<E>, c: &mut E::Context) {
        if b.not_empty() {
            let color = style.color(variant);
            let g = text.as_ref().iter_glyphs();
            self.render_glyphs(*b,inner_offset,color.into().v,g.cloned()).expect("TTOOF");
        }
    }
    #[inline]
    fn set_cursor(&mut self, b: &Bounds, cursor: ESCursor<E>) {
        self.cursor = to_sdl_cursor(cursor);
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
