use guion::style::standard::cursor::StdCursor;

use super::*;

impl Default for Style {
    #[inline]
    fn default() -> Self {
        Self{
            font: None,
            cursor: StdCursor::Arrow,
        }
    }
}
