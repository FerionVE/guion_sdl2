use super::*;
use guion::env::ValidState;

pub struct SimpleValidState {
    pub rerender: bool,
    pub relayout: bool,
}
//TODO move as StandardValidState to guion
impl ValidState for SimpleValidState {
    fn valid() -> Self {
        Self{
            rerender: false,
            relayout: false,
        }
    }
    fn rerender(&self) -> bool {
        self.rerender
    }
    fn relayout(&self) -> bool {
        self.relayout
    }
}
