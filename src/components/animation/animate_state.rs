use crate::{components::Context, Component};

pub struct AnimateState<Func> {
    func: Func,
}

impl<T, Func: Fn(f32, &mut T)> Component<&T, &mut T> for AnimateState<Func> {
    type Input = Func;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { func: input }
    }

    fn process<'c>(&mut self, ctx: &Context, state: &'c mut T) -> &'c mut T {
        let frame_time = ctx.get_delta();
        (self.func)(frame_time, state);
        state
    }
}
