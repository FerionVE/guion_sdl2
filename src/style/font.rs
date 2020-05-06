use sdl2::pixels::Color;
use std::path::Path;
use guion::util::bounds::Offset;
use guion::util::bounds::Dims;
use guion::style::font::*;
use guion::backend::Backend;
use super::*;
use rusttype::*;
#[derive(Clone,PartialEq)]
pub struct Font {
    pub source: FontSource,
    pub index: u32,
    pub size: u16,
    //pub style: FontStyle, //TODO fix
    pub render: FontRender,
}
#[derive(Clone,PartialEq)]
pub enum FontSource {
    File(&'static Path),
    Memory(&'static [u8]),
}
#[derive(Clone,PartialEq)]
pub enum FontRender {
    Solid(),
    Shaded(),
    Blended(),
    BlendedWrapped(Color,u32),
}

pub struct PPText {
    lines: Vec<(Vec<PositionedGlyph<'static>>,Rect<f32>)>,
    size: Vector<f32>,
}

impl<E> PreprocessedText<E> for PPText where
    E: Env + EnvFlexStyleVariant + Sync,
    EStyle<E>: GuionStyle<E,PreprocessedText=Self>,
    E::Context: AsRefMut<Core<E>>
{
    fn size(&self) -> Dims { 
        Dims{
            w: self.size.x as u32,
            h: self.size.y as u32,
        }
    }
    fn lines<'s>(&'s self) -> CrazyWorkaroundPPIter<'s> {
        let iter = self.lines.iter()
            .map(|(chars,size)| {
            (
                Box::new(
                    chars.iter()
                    .map(|pg| {
                        pg.pixel_bounding_box().map(|bb|
                            Bounds::from_xywh(bb.min.x,bb.min.y,bb.width() as u32,bb.height() as u32) //TODO fix sign conversion
                        )
                    })
                ) as Box<dyn Iterator<Item=Option<Bounds>>>,
                Bounds::from_xywh(size.min.x as i32,size.min.y as i32,size.width() as u32,size.height() as u32),
            )
        });
        Box::new(iter)
    }
    fn generate(s: &str, b: (f32,f32), ctx: &mut E::Context) -> Self {
        let font = &ctx.as_ref().font;
        let scale = Scale{x: b.0, y: b.1};
        let mut result = Vec::new();
        let mut current_line = Vec::new();
        let v_metrics = font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;
        let mut max_x = 0f32;
        for c in s.chars() {
            if c.is_control() {
                match c {
                    '\r' | '\n' => {
                        max_x = max_x.max(caret.x);
                        caret = point(0.0, caret.y + advance_height);
                        let rect = Rect{
                            min: Point{x: 0.0, y: 0.0}, //TODO
                            max: Point{x: 0.0, y: 0.0},
                        };
                        result.push((current_line,rect));
                        current_line = Vec::new();
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
                /*if bb.max.x > width as i32 {
                    caret = point(0.0, caret.y + advance_height);
                    glyph.set_position(caret);
                    last_glyph_id = None;
                }*/
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            current_line.push(glyph);
        }
        let rect = Rect{
            min: Point{x: 0.0, y: 0.0}, //TODO
            max: Point{x: 0.0, y: 0.0},
        };
        result.push((current_line,rect));
        max_x = max_x.max(caret.x);
        let bounds = Vector{x: max_x, y: caret.y-v_metrics.descent};
        Self{
            lines: result,
            size: bounds,
        }
    }
}

impl PPText {
    pub fn iter_glyphs(&self) -> impl Iterator<Item=&PositionedGlyph<'static>> {
        self.lines.iter()
        .flat_map(|l| &l.0 )
        //.map(|c| c )
    }
}

impl AsRefMut<Self> for PPText {
    fn as_ref(&self) -> &Self {
        self
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}