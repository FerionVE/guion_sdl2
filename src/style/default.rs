use super::*;
use sdl2::mouse::SystemCursor;

impl Default for Style {
    fn default() -> Self {
        Self{
            inner: StyleInner{
                font: None,
                cursor: Cursor{v: SystemCursor::Arrow}
            },
            current_color: Color::from_rgba8([127,0,0,255]),
        }
    }
}