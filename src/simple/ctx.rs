use guion::core::ctx::widgets::Widgets;
use guion::core::ctx::Context;
use guion::standard::ctx::StandardCtx;
use crate::handler::Handler;
use crate::simple::env::SimpleEnv;
use guion::core::widget::Widget;
use crate::simple::env::SimpleID;
use std::collections::HashMap;

pub struct SimpleContext {
    pub widgets: HashMap<SimpleID,Box<dyn Widget<SimpleEnv>>>,
    pub handler: StandardCtx<Handler<(),Self>,Self>,
}

impl Context<SimpleEnv> for SimpleContext {
    type Handler = ();
    type Queue = ();

    fn queue_mut(&mut self) -> &mut Self::Queue { todo!() }
    fn queue(&self) -> &Self::Queue { todo!() }
    fn _handler_mut(&mut self) -> &mut Self::Handler { todo!() }
    fn _handler(&self) -> &Self::Handler { todo!() }
}

impl Widgets<SimpleEnv> for SimpleContext {
    fn widget(&self, i: &SimpleID) -> Option<&dyn Widget<SimpleEnv>> {
        todo!()
    }
    fn widget_mut(&mut self, i: &SimpleID) -> Option<&mut dyn Widget<SimpleEnv>> {
        todo!()
    }
}