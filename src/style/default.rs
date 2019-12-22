use super::*;

pub trait StyleDefaults: Sized {
    const DEFAULT: Style<Self>;
    const DEFAULT_BORDER: Border;
}