use macroquad::{hash, prelude::Vec2};

use crate::{components::Context, Component};

pub struct GroupProperties<T> {
    size: Vec2,
    extra_data: T,
}

pub struct Group<Child> {
    child: Child,
}
impl<T, Child: for<'z> Component<&'z T, &'z mut T>>
    Component<&GroupProperties<T>, &mut GroupProperties<T>> for Group<Child>
{
    type Input = Child;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { child: input }
    }

    fn process<'c>(
        &mut self,
        context: &Context,
        state: &'c mut GroupProperties<T>,
    ) -> &'c mut GroupProperties<T> {
        {
            self.child.process(context, &mut state.extra_data);
        }
        state
    }

    fn render(&self, context: &Context, props: &GroupProperties<T>) {
        self.child.render(context, &props.extra_data);
    }

    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut GroupProperties<T>,
    ) -> &'c mut GroupProperties<T> {
        ui.group(hash!(), state.size, |ui| {
            self.child.ui(context, ui, &mut state.extra_data);
        });
        state
    }
}
