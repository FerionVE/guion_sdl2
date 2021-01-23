use super::*;
use guion::{handler::standard::StdHandler, state::dyn_state::DynState};
use crate::handler::Handler;
use crate::simple::env::SimpleEnv;
use guion::{ctx::clipboard::CtxClipboardAccess, state::CtxStdState};
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

impl GContext<SimpleEnv> for SimpleCtx {
    type Handler = SimpleHandler;
    type Queue = Queue<SimpleEnv>;

    #[inline]
    fn queue_mut(&mut self) -> &mut Self::Queue {
        &mut self.queue
    }
    #[inline]
    fn queue(&self) -> &Self::Queue {
        &self.queue
    }
}

impl AsRefMut<SimpleHandler> for SimpleCtx {
    #[inline]
    fn as_ref(&self) -> &SimpleHandler {
        &self.handler
    }
    #[inline]
    fn as_mut(&mut self) -> &mut SimpleHandler {
        &mut self.handler
    }
}
impl AsRefMut<Core<SimpleEnv>> for SimpleCtx {
    #[inline]
    fn as_ref(&self) -> &Core<SimpleEnv> {
        &self.handler.sup.inner
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Core<SimpleEnv> {
        &mut self.handler.sup.inner
    }
}
impl CtxStdState<SimpleEnv> for SimpleCtx {
    type T = SimpleHandler;
    #[inline]
    fn state_mut(&mut self) -> &mut Self::T {
        &mut self.handler
    }
    #[inline]
    fn state(&self) -> &Self::T {
        &self.handler
    }
}

impl Deref for SimpleCtx {
    type Target = Core<SimpleEnv>;
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        AsRefMut::as_ref(self)
    }
}
impl DerefMut for SimpleCtx {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        AsRefMut::as_mut(self)
    }
}

//TODO move to handler of different
impl CtxClipboardAccess<SimpleEnv> for SimpleCtx {
    #[inline]
    fn clipboard_set_text(&mut self, v: &str) {
        self.clipboard.set_clipboard_text(v).unwrap();
    }
    #[inline]
    fn clipboard_get_text(&mut self) -> Option<String> {
        self.clipboard.has_clipboard_text()
            .map(|| self.clipboard.clipboard_text().unwrap() )
    }
}

impl DynState<SimpleEnv> for SimpleCtx {
    fn remote_state_or_default<T>(&self, id: StdID) -> T where T: Default + Clone + 'static {
        self.handler.remote_state_or_default(id)
    }
    fn push_remote_state<T>(&mut self, id: StdID, v: T) where T: 'static {
        self.handler.push_remote_state(id,v)
    }
}
