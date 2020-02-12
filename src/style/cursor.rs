use super::*;
use sdl2::mouse::SystemCursor;

#[derive(Clone,PartialEq)]
pub struct Cursor {
    pub v: SystemCursor,
}

/*impl Clone for Cursor {
    fn clone(&self) -> Self {
        Self{
            v: self.v.clone(),
        }
    }
}

impl PartialEq for Cursor {
    fn eq(&self, o: &Self) -> bool {
        todo!()
    }
}*/