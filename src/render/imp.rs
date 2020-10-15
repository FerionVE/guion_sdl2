use crate::style::color::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use guion::{style::variant::standard::StdCursor, render::widgets::RenderStdWidgets};
use super::*;
use style::{cursor::to_sdl_cursor, Style, font::Glyphs};
use std::sync::atomic::{Ordering, AtomicU8};

pub static LOK: AtomicU8 = AtomicU8::new(42);

impl<E> GuionRender<E> for Render<E> where E: Env, ERenderer<E>: AsRefMut<Self> {
    #[inline]
    fn _style(&self) -> &ESVariant<E> {
        &self.live_style
    }
    #[inline]
    fn _bounds(&self) -> &Bounds {
        &self.live_bounds
    }
    #[inline]
    fn _viewport(&self) -> &Bounds {
        &self.live_viewport
    }
    #[inline(never)]
    fn _set_style(&mut self, v: &ESVariant<E>) {
        let a = LOK.load(Ordering::Acquire);
        self.live_style = v.clone();
        LOK.store(a, Ordering::Release);
    }
    #[inline(never)]
    fn _set_bounds(&mut self, v: &Bounds) {
        let a = LOK.load(Ordering::Acquire);
        self.live_bounds = v.clone();
        LOK.store(a, Ordering::Release);
    }
    #[inline(never)]
    fn _set_viewport(&mut self, v: &Bounds) {
        let a = LOK.load(Ordering::Acquire);
        self.live_viewport = v.clone();
        let r = &mut self.windows[self.current];
        r.set_viewport(to_rect(&self.live_viewport));
        LOK.store(a, Ordering::Release);
    }
}

impl<E> RenderStdWidgets<E> for Render<E> where
    E: Env + Sync,
    ERenderer<E>: AsRefMut<Self>,
    EStyle<E>: AsRefMut<Style>,
    ESGlyphs<E>: AsRefMut<Glyphs>,
    ESColor<E>: Into<Color>,
    ESCursor<E>: Into<StdCursor>,
    E::Context: AsRefMut<Core<E>>,
{
    #[inline]
    fn fill_rect(&mut self, c: &mut E::Context) {
        let a = LOK.load(Ordering::Acquire);
        let b = self.live_sdl2_rect();
        let color = c.style_provider().color(&self.live_style);

        if let Some(b) = to_rect(&b) {
            let r = &mut self.windows[self.current];
            r.set_blend_mode(BlendMode::None);
            r.set_draw_color(color.into().v);
            r.fill_rect(Some(b)).expect("SDL Render Failure @ fill_rect");
        }
        LOK.store(a, Ordering::Release);
    }
    #[inline]
    fn fill_border_inner(&mut self, c: &mut E::Context) {
        let a = LOK.load(Ordering::Acquire);
        let b = self.live_sdl2_rect();
        let color = c.style_provider().color(&self.live_style);
        let thickness = c.style_provider().border(&self.live_style).top;

        if thickness == 0 {return;}
        let x = &mut self.windows[self.current];
        x.set_blend_mode(BlendMode::None);
        x.set_draw_color(color.into().v);
        for i in 0..thickness {
            if let Some(r) = to_rect(&b.step(i as i32)) {
                x.draw_rect(r).expect("SDL Render Failure @ draw_rect");
            }
        }
        LOK.store(a, Ordering::Release);
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
    fn render_preprocessed_text(&mut self, text: &ESGlyphs<E>, inner_offset: Offset, c: &mut E::Context) {
        let a = LOK.load(Ordering::Acquire);
        let b = self.live_sdl2_rect();

        if b.not_empty() {
            let color = c.style_provider().color(&self.live_style);
            let g = text.as_ref().iter_glyphs();
            self.render_glyphs(b,inner_offset,color.into().v,g.cloned()).expect("TTOOF");
        }
        LOK.store(a, Ordering::Release);
    }
    #[inline]
    fn set_cursor(&mut self, c: &mut E::Context) {
        let a = LOK.load(Ordering::Acquire);
        let cursor = c.style_provider().cursor(&self.live_style);
        self.cursor = to_sdl_cursor(cursor);
        LOK.store(a, Ordering::Release);
    }
}
#[inline]
pub fn to_rect(b: &Bounds) -> Option<Rect> {
    Some(Rect::new(b.off.x,b.off.y,b.size.w,b.size.h))
    /*if b.size.w > 0 && b.size.h > 0 {
        Some(Rect::new(b.off.x,b.off.y,b.size.w,b.size.h))
    }else{
        None
    }*/
}
