use super::*;
use guion::handler::standard::StdHandler;
use crate::handler::Handler;
use crate::simple::env::SimpleEnv;
use guion::{ctx::clipboard::CtxClipboardAccess, state::handler::AsHandlerStateful};
use crate::core::Queue;
use style::Style;
use qwutils::imp::boolext::BoolExtOption;

pub struct SimpleCtx {
    pub handler: StdHandler<Handler<(),SimpleEnv>,SimpleEnv>,
}

impl SimpleCtx {
    pub fn from_sdl2(sdl: Sdl) -> Result<Self,String> {
        Ok(
            Self{
                handler: StdHandler::new(
                    Handler::new(
                        Core::from_sdl2(sdl)?,
                        (),
                    )
                )
            }
        )
    }
}

pub type SimpleHandler = StdHandler<Handler<(),SimpleEnv>,SimpleEnv>;

impl GuionContext<SimpleEnv> for SimpleCtx {
    type Handler = SimpleHandler;
    type Queue = Queue<SimpleEnv>;

    fn queue_mut(&mut self) -> &mut Self::Queue {
        &mut self.queue
    }
    fn queue(&self) -> &Self::Queue {
        &self.queue
    }
    fn _handler_mut(&mut self) -> &mut Self::Handler {
        &mut self.handler
    }
    fn _handler(&self) -> &Self::Handler {
        &self.handler
    }
    fn default_style(&self) -> &Style {
        &self.default_style
    }
    fn default_border(&self) -> &Border {
        &self.default_border
    }
}

impl AsRefMut<SimpleHandler> for SimpleCtx {
    fn as_ref(&self) -> &SimpleHandler {
        &self.handler
    }
    fn as_mut(&mut self) -> &mut SimpleHandler {
        &mut self.handler
    }
}
impl AsRefMut<Core<SimpleEnv>> for SimpleCtx {
    fn as_ref(&self) -> &Core<SimpleEnv> {
        &self.handler.sup.inner
    }
    fn as_mut(&mut self) -> &mut Core<SimpleEnv> {
        &mut self.handler.sup.inner
    }
}
impl AsHandlerStateful<SimpleEnv> for SimpleCtx {
    type T = SimpleHandler;
    fn stateful_mut(&mut self) -> &mut Self::T {
        &mut self.handler
    }
    fn stateful(&self) -> &Self::T {
        &self.handler
    }
}

impl Deref for SimpleCtx {
    type Target = Core<SimpleEnv>;
    
    fn deref(&self) -> &Self::Target {
        AsRefMut::as_ref(self)
    }
}
impl DerefMut for SimpleCtx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        AsRefMut::as_mut(self)
    }
}

//TODO move to handler of different
impl CtxClipboardAccess<SimpleEnv> for SimpleCtx {
    fn clipboard_set_text(&mut self, v: &str) {
        self.clipboard.set_clipboard_text(v).unwrap();
    }
    fn clipboard_get_text(&mut self) -> Option<String> {
        self.clipboard.has_clipboard_text()
            .map(|| self.clipboard.clipboard_text().unwrap() )
    }
}
