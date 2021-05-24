use guion::ctx::queue::*;
use guion::id::standard::StdID;
use guion::util::sma::SMA;
use guion::widget::as_widget::{AsWidget, AsWidgetMut};
use guion::widget::link::Link;
use guion::widget::resolvable::{Resolvable, ResolvableMut};
use guion::widgets::button::Button;
use guion::widgets::label::Label;
use guion::{const_std_id, constraint};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_sdl2::simple::Simplion;
use guion_sdl2::simple::ctx::SimpleCtx;
use guion_sdl2::simple::env::SimpleEnv;
use guion_sdl2::simple::stor::SimpleStor;

pub struct Model {
    count: u32,
}

const_std_id!(RootE PaneID LabelID ButtonID ButtonLabelID);

impl AsWidget<SimpleEnv> for Model {
    fn as_ref(&self) -> Resolvable<SimpleEnv> {
        Resolvable::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    Label::immediate(LabelID(),self.count)
                        .with_size(constraint!(~0-@2.0|24)),
                    Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment".to_owned())),
                ),
            )
        )
    }
    fn into_ref<'w>(self) -> Resolvable<'w,SimpleEnv> where Self: 'w {
        unimplemented!()
    }
}
impl AsWidgetMut<SimpleEnv> for Model {
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<SimpleEnv> {
        let cs = self.count.to_string();
        let c = &mut self.count;
        let increment = move |_: &mut _| *c += 1; // https://github.com/rust-lang/rust/issues/81511

        ResolvableMut::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    Label::immediate(LabelID(),cs/*HACK*/),
                    Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment".to_owned()))
                        .with_trigger_mut(increment),
                ),
            )
        )
    }
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> where Self: 'w {
        unimplemented!()
    }
}

fn main() {
    let mut simplion = Simplion::new();

    let window = simplion.ctx
        .video
        .window("7gui1", 400, 80)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    simplion.push_window(window,Model{count:0});

    while simplion.do_events() {}
}