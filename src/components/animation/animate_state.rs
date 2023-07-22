use macroquad::time::get_frame_time;

use crate::Component;

pub struct AnimateState<Func> {
    func: Func,
}

impl<T, Func: Fn(f32, &mut T)> Component<T> for AnimateState<Func> {
    type Input = Func;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { func: input }
    }

    fn process(&mut self, state: &mut T) {
        let frame_time = get_frame_time();
        (self.func)(frame_time, state)
    }

    fn render(&self, _: &T) {}

    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut T) {}
}
