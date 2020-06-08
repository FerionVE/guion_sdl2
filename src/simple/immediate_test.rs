use super::*;
use guion::{widgets::{textbox::{state::Cursor, TextBox}, label::Label}, widget::{resolvable::{ResolvableMut, Resolvable}, as_widget::{AsWidgetMut, AsWidget}}};

use guion::widget::WBaseMut; //baka import
use guion::widget::WBase;

impl<'w> AsWidget<'w,SimpleEnv> for (String,StdID) {
    fn as_ref<'s>(&'s self) -> guion::widget::resolvable::Resolvable<'s,SimpleEnv> where 'w: 's {
        Resolvable::Widget(
            Label::new(self.1.clone())
            .with_text(&self.0)
            .boxed_ref()
        )
    }
    fn into_ref(self) -> guion::widget::resolvable::Resolvable<'w,SimpleEnv> {
        todo!()
    }
}

impl<'w> AsWidgetMut<'w,SimpleEnv> for (String,StdID) {
    fn as_mut<'s>(&'s mut self) -> guion::widget::resolvable::ResolvableMut<'s,SimpleEnv> where 'w: 's {
        ResolvableMut::Widget(
            Label::new(self.1.clone())
            .with_text(&mut self.0)
            .boxed()
        )
    }
    fn into_mut(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> {
        todo!()
    }
}

impl<'w> AsWidget<'w,SimpleEnv> for (String,StdID,(u32,u32),Cursor,Option<u32>) {
    fn as_ref<'s>(&'s self) -> guion::widget::resolvable::Resolvable<'s,SimpleEnv> where 'w: 's {
        Resolvable::Widget(
            TextBox::new(self.1.clone())
            .with_text(&self.0)
            .with_states(&self.2, &self.3, &self.4)
            .boxed_ref()
        )
    }
    fn into_ref(self) -> guion::widget::resolvable::Resolvable<'w,SimpleEnv> {
        todo!()
    }
}

impl<'w> AsWidgetMut<'w,SimpleEnv> for (String,StdID,(u32,u32),Cursor,Option<u32>) {
    fn as_mut<'s>(&'s mut self) -> guion::widget::resolvable::ResolvableMut<'s,SimpleEnv> where 'w: 's {
        ResolvableMut::Widget(
            TextBox::new(self.1.clone())
            .with_text(&mut self.0)
            .with_states(&mut self.2, &mut self.3, &mut self.4)
            .boxed()
        )
    }
    fn into_mut(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> {
        todo!()
    }
}