use sdl2::{surface::SurfaceRef, rect::Rect, video::Window, render::Canvas};

pub trait RenderSurface {
    fn render_surface(&mut self, dest: Rect, src: &SurfaceRef, src_box: Rect) -> Result<(),String>;
}

impl RenderSurface for Canvas<Window> {
    fn render_surface(&mut self, dest: Rect, src: &SurfaceRef, src_box: Rect) -> Result<(),String> {
        let t = self.texture_creator();
        let t = t
            .create_texture_from_surface(src)
            .map_err(|e| e.to_string() )?;
        
        self.copy(&t,src_box,dest)
    }
}