use std::fmt::Debug;

use macroquad::{
    prelude::Vec2,
    ui::{widgets, UiContent},
};

use crate::Component;

pub struct ButtonProperties<'a, T> {
    pub size: Vec2,
    pub selected: bool,
    pub position: Vec2,
    pub extra_data: T,
    pub content: UiContent<'a>,
}

impl<'a, T: Debug> Debug for ButtonProperties<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonProperties")
            .field("size", &self.size)
            .field("selected", &self.selected)
            .field("position", &self.position)
            .field("extra_data", &self.extra_data)
            .finish()
    }
}

pub struct Button<OnClick> {
    on_click: OnClick,
}

impl<'a, T, OnClick: Fn(&mut ButtonProperties<'a, T>)> Component<ButtonProperties<'a, T>>
    for Button<OnClick>
{
    type Input = OnClick;

    fn instantiate(on_click: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { on_click }
    }

    fn process(&mut self, _: &mut ButtonProperties<'a, T>) {}

    fn render(&self, _: &ButtonProperties<'a, T>) {}

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut ButtonProperties<'a, T>) {
        let x = match &state.content {
            UiContent::Label(x) => UiContent::Label(std::borrow::Cow::Borrowed(x)),
            UiContent::Texture(x) => UiContent::Texture(x.clone()),
        };
        let x = widgets::Button::new(x)
            .position(state.position)
            .selected(state.selected)
            .size(state.size)
            .ui(ui);
        if x {
            (self.on_click)(state);
        }
    }
}
