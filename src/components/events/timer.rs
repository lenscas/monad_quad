use std::marker::PhantomData;

use macroquad::time::get_frame_time;

use crate::components::Component;

pub struct Timer<State, OnReachedTime: Fn(&mut State)> {
    _state: PhantomData<State>,
    max_time: f32,
    current_time: f32,
    on_reached_time: OnReachedTime,
}

impl<State, OnReachedTime: Fn(&mut State)> Timer<State, OnReachedTime> {
    pub fn new(max_time: f32, on_reached_time: OnReachedTime) -> Self {
        Self {
            _state: PhantomData,
            max_time,
            current_time: 0.,
            on_reached_time,
        }
    }
}

impl<State, OnReachedTime: Fn(&mut State)> Component<State> for Timer<State, OnReachedTime> {
    type Input = (f32, OnReachedTime);

    fn instantiate((max_time, on_reached_time): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(max_time, on_reached_time)
    }

    fn process(&mut self, state: &mut State) {
        let frame_time = get_frame_time();
        self.current_time += frame_time;
        while self.current_time > self.max_time {
            (self.on_reached_time)(state);
            self.current_time -= self.max_time
        }
    }

    fn render(&self, _: &State) {}
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut State) {}
}

pub struct VariableTimer<State, GetMaxTime: Fn(&State) -> f32, OnReachedTime: Fn(&mut State)> {
    _state: PhantomData<State>,
    current_time: f32,
    on_reached_time: OnReachedTime,
    get_max_time: GetMaxTime,
}

impl<State, GetMaxTime: Fn(&State) -> f32, OnReachedTime: Fn(&mut State)>
    VariableTimer<State, GetMaxTime, OnReachedTime>
{
    pub fn new(get_max_time: GetMaxTime, on_reached_time: OnReachedTime) -> Self {
        Self {
            _state: PhantomData,
            get_max_time,
            current_time: 0.,
            on_reached_time,
        }
    }
}

impl<State, GetMaxTime: Fn(&State) -> f32, OnReachedTime: Fn(&mut State)> Component<State>
    for VariableTimer<State, GetMaxTime, OnReachedTime>
{
    type Input = (GetMaxTime, OnReachedTime);

    fn instantiate((max_time, on_reached_time): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(max_time, on_reached_time)
    }

    fn process(&mut self, state: &mut State) {
        let frame_time = get_frame_time();
        self.current_time += frame_time;
        let mut max_time = (self.get_max_time)(state);
        while self.current_time > max_time {
            (self.on_reached_time)(state);
            max_time = (self.get_max_time)(state);
            self.current_time -= max_time
        }
    }

    fn render(&self, _: &State) {}
    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut State) {}
}
