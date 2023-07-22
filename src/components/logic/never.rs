use crate::Component;

pub struct Never;

impl<T> Component<T> for Never {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process(&mut self, _: &mut T) {}

    fn render(&self, _: &T) {}

    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut T) {}
}
