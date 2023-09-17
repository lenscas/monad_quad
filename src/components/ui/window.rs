use context::Context;
use macroquad::{prelude::Vec2, ui::widgets};

use crate::{components::context, Component};

#[derive(Clone, Debug)]
pub struct WindowProperties<T> {
    pub location: Vec2,
    pub size: Vec2,
    pub label: Option<String>,
    pub title_bar: bool,
    pub extra_data: T,
    pub moveable: bool,
    pub close_button: bool,
}

pub struct Window<Child> {
    child: Child,
    id: u64,
}

impl<Child> Window<Child> {
    pub fn new<T>(id: u64, child: Child) -> Self
    where
        Self: Sized,
        Child: for<'z> Component<&'z T, &'z mut T>,
    {
        Self::instantiate((id, child))
    }
}

impl<T, Child: for<'z> Component<&'z T, &'z mut T>>
    Component<&WindowProperties<T>, &mut WindowProperties<T>> for Window<Child>
{
    type Input = (u64, Child);

    fn instantiate((id, child): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { child, id }
    }

    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut WindowProperties<T>,
    ) -> &'c mut WindowProperties<T> {
        let mut window = widgets::Window::new(self.id, state.location, state.size)
            .titlebar(state.title_bar)
            .movable(state.moveable)
            .close_button(state.close_button);
        if let Some(label) = &state.label {
            window = window.label(label);
        }

        window.ui(ui, |ui| {
            self.child.ui(context, ui, &mut state.extra_data);
        });
        state
    }
}
