use super::*;
use sdl2::mouse::SystemCursor;

impl Default for Style {
    fn default() -> Self {
        Self{
            font: None,
            cursor: StdCursor::Arrow,
        }
    }
}