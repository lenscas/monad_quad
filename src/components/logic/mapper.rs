use std::marker::PhantomData;

use crate::Component;

use super::Selector;

/// Helper to instantiate components if their properties is not perfectly aligned with the current available state

pub struct Comp<State, T: Component<State>> {
    _state: PhantomData<State>,
    _t: PhantomData<T>,
}
impl<State, T: Component<State>> Comp<State, T> {
    /// builds the component as is, without any mapping
    pub fn build(a: T::Input) -> T {
        T::instantiate(a)
    }
    /// Configure the function needed to go from State A to the properties needed for Component B

    pub fn map_in<StateIn, Func: Fn(&StateIn) -> State>(
        func: Func,
    ) -> MapInto<StateIn, State, T, Func> {
        MapInto {
            func,
            _state_in: PhantomData,
            _state_out: PhantomData,
            _t: PhantomData,
        }
    }
}
/// constructed by [Comp<State,Component>::map_in](crate::components::Comp)
///
/// Allows you to construct the component by setting how the output of the Component maps back into the given state
pub struct MapInto<StateIn, StateOut, T: Component<StateOut>, MapIn: Fn(&StateIn) -> StateOut> {
    _state_in: PhantomData<StateIn>,
    _state_out: PhantomData<StateOut>,
    _t: PhantomData<T>,
    func: MapIn,
}
impl<StateIn, StateOut, T: Component<StateOut>, MapIn: Fn(&StateIn) -> StateOut>
    MapInto<StateIn, StateOut, T, MapIn>
{
    /// constructs the component with the given initialized data
    pub fn map_out_with<FuncOut: Fn(StateOut, &mut StateIn)>(
        self,
        input: T::Input,
        func: FuncOut,
    ) -> Selector<StateIn, StateOut, MapIn, FuncOut, T> {
        Selector::new(self.func, func, T::instantiate(input))
    }
    ///uses an already existing component
    pub fn map_out_for<FuncOut: Fn(StateOut, &mut StateIn)>(
        self,
        func: FuncOut,
        child: T,
    ) -> Selector<StateIn, StateOut, MapIn, FuncOut, T> {
        Selector::new(self.func, func, child)
    }
}
impl<StateIn, StateOut, T: Component<StateOut>, MapIn: Fn(&StateIn) -> StateOut>
    MapInto<StateIn, StateOut, T, MapIn>
where
    T::Input: Default,
{
    /// constructs the component using the default data for its initialized data
    pub fn map_out<FuncOut: Fn(StateOut, &mut StateIn)>(
        self,
        func: FuncOut,
    ) -> Selector<StateIn, StateOut, MapIn, FuncOut, T> {
        Selector::new(self.func, func, T::instantiate(Default::default()))
    }
}
