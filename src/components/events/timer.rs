use macroquad::time::get_frame_time;

use crate::components::{Component, Context};

pub struct Timer<OnReachedTime> {
    max_time: f32,
    current_time: f32,
    on_reached_time: OnReachedTime,
}

impl<OnReachedTime> Timer<OnReachedTime> {
    pub fn new<ProcessState>(max_time: f32, on_reached_time: OnReachedTime) -> Self
    where
        OnReachedTime: Fn(&mut ProcessState),
    {
        Self {
            max_time,
            current_time: 0.,
            on_reached_time,
        }
    }
}

impl<RenderState: Clone, ProcessState, OnReachedTime: Fn(&mut ProcessState)>
    Component<RenderState, ProcessState> for Timer<OnReachedTime>
{
    type Input = (f32, OnReachedTime);

    fn instantiate((max_time, on_reached_time): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(max_time, on_reached_time)
    }

    fn process(&mut self, _: &Context, mut state: ProcessState) -> ProcessState {
        let frame_time = get_frame_time();
        self.current_time += frame_time;
        while self.current_time > self.max_time {
            (self.on_reached_time)(&mut state);
            self.current_time -= self.max_time
        }
        state
    }
}

pub struct VariableTimer<GetMaxTime, OnReachedTime> {
    current_time: f32,
    on_reached_time: OnReachedTime,
    get_max_time: GetMaxTime,
}

impl<GetMaxTime, OnReachedTime> VariableTimer<GetMaxTime, OnReachedTime> {
    pub fn new<ProcessState>(get_max_time: GetMaxTime, on_reached_time: OnReachedTime) -> Self
    where
        GetMaxTime: Fn(&ProcessState) -> f32,
        OnReachedTime: Fn(&Context, &mut ProcessState),
    {
        Self {
            get_max_time,
            current_time: 0.,
            on_reached_time,
        }
    }
}

impl<X: Clone, State, GetMaxTime: Fn(&State) -> f32, OnReachedTime: Fn(&Context, &mut State)>
    Component<X, &mut State> for VariableTimer<GetMaxTime, OnReachedTime>
{
    type Input = (GetMaxTime, OnReachedTime);

    fn instantiate((max_time, on_reached_time): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(max_time, on_reached_time)
    }

    fn process<'c>(&mut self, ctx: &Context, state: &'c mut State) -> &'c mut State {
        let frame_time = ctx.get_delta();
        self.current_time += frame_time;
        let mut max_time = (self.get_max_time)(state);
        while self.current_time > max_time {
            (self.on_reached_time)(ctx, state);
            max_time = (self.get_max_time)(state);
            self.current_time -= max_time
        }
        state
    }
}
