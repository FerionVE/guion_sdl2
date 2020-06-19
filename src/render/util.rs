use sdl2::{surface::SurfaceRef, rect::Rect, video::Window, render::{Texture, Canvas}, pixels::PixelFormatEnum};

pub trait RenderSurface {
    fn render_surface(&mut self, dest: Rect, src: &SurfaceRef, src_box: Rect) -> Result<(),String>;
    fn create_cache_tex(&mut self, size: (u32,u32), p: PixelFormatEnum) -> Result<Texture<'static>,String>;
}

impl RenderSurface for Canvas<Window> {
    fn render_surface(&mut self, dest: Rect, src: &SurfaceRef, src_box: Rect) -> Result<(),String> {
        let t = self.texture_creator();
        let t = t
            .create_texture_from_surface(src)
            .map_err(|e| e.to_string() )?;
        
        self.copy(&t,src_box,dest)
    }
    fn create_cache_tex(&mut self, size: (u32,u32), p: PixelFormatEnum) -> Result<Texture<'static>,String> {
        let t = self.texture_creator();
        let t = Box::leak(Box::new(t));
        t
            .create_texture_static(p, size.0, size.1)
            .map_err(|e| e.to_string() )
    }
}
