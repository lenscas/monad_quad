use macroquad::{hash, prelude::Vec2};

use crate::Component;

pub struct GroupProperties<T> {
    size: Vec2,
    extra_data: T,
}

pub struct Group<Child> {
    child: Child,
}
impl<T, Child: Component<T>> Component<GroupProperties<T>> for Group<Child> {
    type Input = Child;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { child: input }
    }

    fn process(&mut self, state: &mut GroupProperties<T>) {
        self.child.process(&mut state.extra_data)
    }

    fn render(&self, props: &GroupProperties<T>) {
        self.child.render(&props.extra_data);
    }

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut GroupProperties<T>) {
        ui.group(hash!(), state.size, |ui| {
            self.child.ui(ui, &mut state.extra_data)
        });
    }
}
