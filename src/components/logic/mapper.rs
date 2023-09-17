use std::marker::PhantomData;

use crate::Component;

use super::Selector;

/// Helper to instantiate components if their properties is not perfectly aligned with the current available state

pub struct Comp<T, C> {
    _state: PhantomData<T>,
    _t: PhantomData<C>,
}
impl<State, T: for<'a> Component<&'a State, &'a mut State>> Comp<State, T> {
    /// builds the component as is, without any mapping
    pub fn build<'a>(a: <T as Component<&'a State, &'a mut State>>::Input) -> T {
        T::instantiate(a)
    }
    /// Configure the function needed to go from State A to the properties needed for Component B

    pub fn map_in<StateIn, Func: Fn(&StateIn) -> State>(
        func: Func,
    ) -> MapInto<T, StateIn, State, Func> {
        MapInto {
            _component: PhantomData,
            _state_in: PhantomData,
            _state_out: PhantomData,
            func,
        }
    }
}
/// constructed by [Comp<State,Component>::map_in](crate::components::Comp)
///
/// Allows you to construct the component by setting how the output of the Component maps back into the given state
pub struct MapInto<Comp, StateIn, StateOut, MapIn> {
    _state_in: PhantomData<StateIn>,
    _state_out: PhantomData<StateOut>,
    _component: PhantomData<Comp>,
    func: MapIn,
}
impl<
        T: for<'a> Component<&'a StateOut, &'a mut StateOut>,
        StateIn,
        StateOut,
        MapIn: Fn(&StateIn) -> StateOut,
    > MapInto<T, StateIn, StateOut, MapIn>
{
    /// constructs the component with the given initialized data
    pub fn map_out_with<'a, FuncOut: Fn(StateOut, &mut StateIn)>(
        self,
        input: <T as Component<&'a StateOut, &'a mut StateOut>>::Input,
        func: FuncOut,
    ) -> Selector<MapIn, FuncOut, T> {
        Selector::new(self.func, func, T::instantiate(input))
    }
    ///uses an already existing component
    pub fn map_out_for<FuncOut: Fn(StateOut, &mut StateIn)>(
        self,
        func: FuncOut,
        child: T,
    ) -> Selector<MapIn, FuncOut, T> {
        Selector::new(self.func, func, child)
    }
    /// constructs the component using the default data for its initialized data
    pub fn map_out<'a, FuncOut: Fn(StateOut, &mut StateIn)>(
        self,
        func: FuncOut,
    ) -> Selector<MapIn, FuncOut, T>
    where
        StateOut: 'a,
        <T as Component<&'a StateOut, &'a mut StateOut>>::Input: Default,
    {
        Selector::new(self.func, func, T::instantiate(Default::default()))
    }
}
