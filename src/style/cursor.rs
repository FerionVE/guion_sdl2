use sdl2::mouse::Cursor as SDLCursor;

pub struct Cursor {
    pub v: SDLCursor,
}

impl Clone for Cursor {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl PartialEq for Cursor {
    fn eq(&self, o: &Self) -> bool {
        todo!()
    }
}