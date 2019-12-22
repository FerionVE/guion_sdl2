use sdl2::mouse::Cursor as SDLCursor;
use super::*;

pub struct Cursor {
    pub v: SDLCursor,
}

impl Clone for Cursor {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl PartialEq for Cursor {
    fn eq(&self, o: &Self) -> bool {
        unimplemented!()
    }
}