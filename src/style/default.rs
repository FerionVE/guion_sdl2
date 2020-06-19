use super::*;

impl Default for Style {
    fn default() -> Self {
        Self{
            font: None,
            cursor: StdCursor::Arrow,
        }
    }
}
