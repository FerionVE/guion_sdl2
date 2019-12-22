use super::*;

pub trait StyleDefaults: Sized + 'static {
    const DEFAULT: &'static Style<Self>;
    const DEFAULT_BORDER: &'static Border;
}