use super::*;
use rusttype::*;
use sdl2::{render::BlendMode, pixels::PixelFormatEnum};
use guion::util::bounds::Dims;

impl<C> Render<C> where C: RenderTarget, Canvas<C>: RenderSurface {
    //TODO: integrate gpu_cache later
    pub fn render_glyphs<G: Into<PositionedGlyph<'static>>>(&mut self, b: Bounds, o: Offset, color: SDLColor, gs: impl Iterator<Item=G>) -> Result<(),String> {
        //check for conservative in-bounds
        fn in_bounds(g: &PositionedGlyph, o: Offset, d: Dims) -> bool {
            if let Some(bb) = g.pixel_bounding_box() {
                let x0 = bb.min.x - o.x;
                let y0 = bb.min.y - o.y;
                let x1 = bb.max.x - o.x;
                let y1 = bb.max.y - o.y;
                //let bb = bb - Vector{x: o.x, y: o.y};

                x1 > 0 && y1 > 0 && x0 < d.w as i32 && y0 < d.h as i32
            }else{
                false
            }
        }

        let gs = gs.map(|g| g.into() ).collect::<Vec<_>>();

        //let bf = Rect{min: Point{x: b.x() as f32, y: b.y() as f32}, max: Point{x: b.x1() as f32, y: b.y1() as f32}};
        
        //generate the texture if none
        if self.cache_tex.is_none() {
            let mut t = self.c.create_cache_tex(self.cache.dimensions(), PixelFormatEnum::ARGB8888)?;
            t.set_blend_mode(BlendMode::Blend);
            self.cache_tex = Some(t);
        }
        let tex_ref = self.cache_tex.as_mut().unwrap();

        //queue the glyphs in the cache
        //feed more glyphs in as the atlas can hold and watch it fail
        for g in &gs {
            if in_bounds(g, o, b.size) {
                self.cache.queue_glyph(0, g.clone());
            }
        }

        //let cache upload to texture
        self.cache.cache_queued(|r,d| {
            let r = sdl2::rect::Rect::new(r.min.x as i32, r.min.y as i32, r.width(), r.height());
            
            //we have to generate BGRA(!) here. Why? only god knows
            let mut data = vec![0xFF;d.len()*4];
            for (i,p) in d.iter().enumerate() {
                data[i*4+3] = *p;
            }

            tex_ref
                .update(r, &data, 1)
                .expect("TODO");
        }).map_err(|e| e.to_string() )?;

        tex_ref.set_color_mod(color.r, color.g, color.b);
        tex_ref.set_alpha_mod(color.a);

        //draw the glyphs
        for g in &gs {
            if in_bounds(g, o, b.size) {
                if let Some((src,dest)) = self.cache.rect_for(0, g).unwrap() {
                    let cache_dims = self.cache.dimensions();
                    let sx = src.min.x * cache_dims.0 as f32;
                    let sy = src.min.y * cache_dims.1 as f32;
                    let sw = src.width() * cache_dims.0 as f32;
                    let sh = src.height() * cache_dims.1 as f32;
                    let dx0 = dest.min.x - o.x + b.off.x;
                    let dy0 = dest.min.y - o.y + b.off.y;
                    //let dx1 = dest.max.x - o.x + b.off.x;
                    //let dy1 = dest.max.y - o.y + b.off.y;
                    //TODO crop to dest bounds
                    let w = dest.width().min( sw as i32 ) as u32;
                    let h = dest.height().min( sh as i32 ) as u32;

                    let src = sdl2::rect::Rect::new(sx as i32, sy as i32, w, h);
                    let dest = sdl2::rect::Rect::new(dx0, dy0, w, h);

                    //eprintln!("{}_{}_{}_{} to {}_{}_{}_{}",src.x(),src.y(),src.width(),src.height(),dest.x(),dest.y(),dest.width(),dest.height());

                    self.c.copy(tex_ref, src, dest)?;
                }
            }
        }
        //self.c.copy(tex_ref, sdl2::rect::Rect::new(0,0,256,256), sdl2::rect::Rect::new(0,0,256,256)).expect("F");
        Ok(())
    }
}

pub fn glyphs_of_str<'a>(
    font: &Font<'a>,
    scale: Scale,
    width: u32,
    text: &str,
) -> (Vec<PositionedGlyph<'a>>,Vector<f32>) {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    let mut max_x = 0f32;
    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' | '\n' => {
                    max_x = max_x.max(caret.x);
                    caret = point(0.0, caret.y + advance_height);
                }
                _ => {}
            }
            continue;
        }
        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    max_x = max_x.max(caret.x);
    let bounds = Vector{x: max_x, y: caret.y-v_metrics.descent};
    (result,bounds)
}

pub fn load_font() -> Font<'static> {
    let data: &'static [u8] = include_bytes!("../../assets/NotoSans-Regular.ttf");
    Font::try_from_bytes(data).expect("Oof Font Failed")
}