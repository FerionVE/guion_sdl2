use super::*;
use guion::{constraint, widget::{resolvable::{ResolvableMut, Resolvable}, as_widget::{AsWidgetMut, AsWidget}}, widgets::{label::Label, textbox::{state::Cursor, TextBox}, util::remote_state::RemoteState}};

use guion::widget::WBaseMut; //baka import
use guion::{validation::validated::Validated, widget::WBase};

pub struct ImmediateLabel {
    pub id: StdID,
    pub text: String,
}
pub struct ImmediateTextBox {
    pub id: StdID,
    pub text: String,
}

impl AsWidget<SimpleEnv> for ImmediateLabel {
    fn as_ref(&self) -> guion::widget::resolvable::Resolvable<SimpleEnv> {
        Resolvable::from_widget(
            Label::immediate(self.id.clone(),&self.text)
                .with_size(constraint!(~0-|24))
        )
    }
    fn into_ref<'w>(self) -> guion::widget::resolvable::Resolvable<'w,SimpleEnv> where Self: 'w {
        Resolvable::from_widget(
            Label::immediate(self.id.clone(),self.text)
                .with_size(constraint!(~0-|24))
        )
    }
}

impl AsWidgetMut<SimpleEnv> for ImmediateLabel {
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<SimpleEnv> {
        ResolvableMut::from_widget(
            Label::immediate(self.id.clone(),&mut self.text)
                .with_size(constraint!(~0-|24))
        )
    }
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> where Self: 'w {
        ResolvableMut::from_widget(
            Label::immediate(self.id.clone(),self.text)
                .with_size(constraint!(~0-|24))
        )
    }
}

impl AsWidget<SimpleEnv> for ImmediateTextBox {
    fn as_ref(&self) -> guion::widget::resolvable::Resolvable<SimpleEnv> {
        Resolvable::from_widget(
            TextBox::immediate(self.id.clone(),&self.text)
        )
    }
    fn into_ref<'w>(self) -> guion::widget::resolvable::Resolvable<'w,SimpleEnv> where Self: 'w {
        Resolvable::from_widget(
            TextBox::immediate(self.id.clone(),self.text)
        )
    }
}

impl AsWidgetMut<SimpleEnv> for ImmediateTextBox {
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<SimpleEnv> {
        ResolvableMut::from_widget(
            TextBox::immediate(self.id.clone(),&mut self.text)
        )
    }
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> where Self: 'w {
        ResolvableMut::from_widget(
            TextBox::immediate(self.id.clone(),self.text)
        )
    }
}
