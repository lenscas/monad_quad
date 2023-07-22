pub mod animation;
pub mod events;
pub mod logic;
pub mod render;
pub mod ui;

use std::{marker::PhantomData, ops::Range};

use self::logic::Eraser;

/// The trait that the scene and state management is built on top off.
///
/// Every node in the scene tree implements this trait
pub trait Component<TIn> {
    /// The type that the component needs to get in order to be instantiated
    type Input;
    /// Instantiates the component
    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized;
    /// Runs once every frame, allows the state to be mutated
    fn process(&mut self, state: &mut TIn);
    /// Runs once every frame to render the current frame
    fn render(&self, props: &TIn);

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut TIn);

    /// Erases the types a bit, may improve compile time performance at the cost of runtime performance
    fn boxed(self) -> Eraser<TIn, Self::Input>
    where
        Self: Sized + 'static,
    {
        Eraser::new(self)
    }
}

/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<State, Comp: Component<State>> Component<State> for (Comp,) {
    fn process(&mut self, state: &mut State) {
        self.0.process(state)
    }

    fn render(&self, props: &State) {
        self.0.render(props)
    }

    type Input = Comp::Input;

    fn instantiate(input: Self::Input) -> Self {
        (Comp::instantiate(input),)
    }

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.0.ui(ui, state)
    }
}
impl<'a, State: 'a, Comp: Component<State>> MultiComponent<State> for (Comp,) {
    fn len(&self) -> usize {
        todo!()
    }

    fn process_part(&mut self, range: Range<usize>, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.process(state)
            } else {
                break;
            }
        }
    }

    fn render_part(&self, range: Range<usize>, state: &State) {
        for v in range {
            if v == 0 {
                self.0.render(state)
            } else {
                break;
            }
        }
    }
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.ui(ui, state)
            } else {
                break;
            }
        }
    }
}

/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<State, Comp: Component<State>, Comp2: Component<State>> Component<State> for (Comp, Comp2) {
    fn process(&mut self, state: &mut State) {
        self.0.process(state);
        self.1.process(state);
    }

    fn render(&self, props: &State) {
        self.0.render(props);
        self.1.render(props)
    }

    type Input = (Comp::Input, Comp2::Input);

    fn instantiate(input: Self::Input) -> Self {
        (Comp::instantiate(input.0), Comp2::instantiate(input.1))
    }

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.0.ui(ui, state);
        self.1.ui(ui, state);
    }
}
impl<'a, State: 'a, Comp: Component<State>, Comp2: Component<State>> MultiComponent<State>
    for (Comp, Comp2)
{
    fn len(&self) -> usize {
        2
    }

    fn process_part(&mut self, range: Range<usize>, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.process(state)
            } else if v == 1 {
                self.1.process(state)
            } else {
                break;
            }
        }
    }

    fn render_part(&self, range: Range<usize>, state: &State) {
        for v in range {
            if v == 0 {
                self.0.render(state)
            } else if v == 1 {
                self.1.render(state)
            } else {
                break;
            }
        }
    }
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.ui(ui, state)
            } else if v == 1 {
                self.1.ui(ui, state)
            } else {
                break;
            }
        }
    }
}
/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<State, Comp: Component<State>, Comp2: Component<State>, Comp3: Component<State>>
    Component<State> for (Comp, Comp2, Comp3)
{
    fn process(&mut self, state: &mut State) {
        self.0.process(state);
        self.1.process(state);
        self.2.process(state)
    }

    fn render(&self, props: &State) {
        self.0.render(props);
        self.1.render(props);
        self.2.render(props)
    }

    type Input = (Comp::Input, Comp2::Input, Comp3::Input);

    fn instantiate(input: Self::Input) -> Self {
        (
            Comp::instantiate(input.0),
            Comp2::instantiate(input.1),
            Comp3::instantiate(input.2),
        )
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.0.ui(ui, state);
        self.1.ui(ui, state);
        self.2.ui(ui, state);
    }
}
impl<'a, State: 'a, Comp: Component<State>, Comp2: Component<State>, Comp3: Component<State>>
    MultiComponent<State> for (Comp, Comp2, Comp3)
{
    fn len(&self) -> usize {
        3
    }

    fn process_part(&mut self, range: Range<usize>, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.process(state)
            } else if v == 1 {
                self.1.process(state)
            } else if v == 2 {
                self.2.process(state)
            } else {
                break;
            }
        }
    }

    fn render_part(&self, range: Range<usize>, state: &State) {
        for v in range {
            if v == 0 {
                self.0.render(state)
            } else if v == 1 {
                self.1.render(state)
            } else if v == 2 {
                self.2.render(state)
            } else {
                break;
            }
        }
    }
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.ui(ui, state)
            } else if v == 1 {
                self.1.ui(ui, state)
            } else if v == 2 {
                self.2.ui(ui, state)
            } else {
                break;
            }
        }
    }
}
/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<
        State,
        Comp: Component<State>,
        Comp2: Component<State>,
        Comp3: Component<State>,
        Comp4: Component<State>,
    > Component<State> for (Comp, Comp2, Comp3, Comp4)
{
    fn process(&mut self, state: &mut State) {
        self.0.process(state);
        self.1.process(state);
        self.2.process(state);
        self.3.process(state);
    }

    fn render(&self, props: &State) {
        self.0.render(props);
        self.1.render(props);
        self.2.render(props);
        self.3.render(props)
    }

    type Input = (Comp::Input, Comp2::Input, Comp3::Input, Comp4::Input);

    fn instantiate(input: Self::Input) -> Self {
        (
            Comp::instantiate(input.0),
            Comp2::instantiate(input.1),
            Comp3::instantiate(input.2),
            Comp4::instantiate(input.3),
        )
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.0.ui(ui, state);
        self.1.ui(ui, state);
        self.2.ui(ui, state);
        self.3.ui(ui, state);
    }
}

impl<
        'a,
        State: 'a,
        Comp: Component<State>,
        Comp2: Component<State>,
        Comp3: Component<State>,
        Comp4: Component<State>,
    > MultiComponent<State> for (Comp, Comp2, Comp3, Comp4)
{
    fn len(&self) -> usize {
        4
    }

    fn process_part(&mut self, range: Range<usize>, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.process(state)
            } else if v == 1 {
                self.1.process(state)
            } else if v == 2 {
                self.2.process(state)
            } else if v == 3 {
                self.3.process(state)
            } else {
                break;
            }
        }
    }

    fn render_part(&self, range: Range<usize>, state: &State) {
        for v in range {
            if v == 0 {
                self.0.render(state)
            } else if v == 1 {
                self.1.render(state)
            } else if v == 2 {
                self.2.render(state)
            } else if v == 3 {
                self.3.render(state)
            } else {
                break;
            }
        }
    }
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut State) {
        for v in range {
            if v == 0 {
                self.0.ui(ui, state)
            } else if v == 1 {
                self.1.ui(ui, state)
            } else if v == 2 {
                self.2.ui(ui, state)
            } else if v == 3 {
                self.3.ui(ui, state)
            } else {
                break;
            }
        }
    }
}

pub trait MultiComponent<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn render_all(&self, state: &T) {
        self.render_part(0..self.len(), state)
    }
    fn ui_all(&mut self, ui: &mut macroquad::ui::Ui, state: &mut T) {
        self.ui_part(0..self.len(), ui, state);
    }
    fn process_all(&mut self, state: &mut T) {
        self.process_part(0..self.len(), state)
    }
    fn process_part(&mut self, range: Range<usize>, state: &mut T);
    fn render_part(&self, range: Range<usize>, state: &T);
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut T);
}

impl<T, Input: Clone, Comp: Component<T, Input = Input>> MultiComponent<T> for Vec<Comp> {
    fn len(&self) -> usize {
        self.len()
    }
    fn process_part(&mut self, range: Range<usize>, state: &mut T) {
        for v in range {
            let part = match self.get_mut(v) {
                None => return,
                Some(x) => x,
            };
            part.process(state)
        }
    }

    fn render_part(&self, range: Range<usize>, state: &T) {
        for v in range {
            let part = match self.get(v) {
                None => return,
                Some(x) => x,
            };
            part.render(state)
        }
    }
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut T) {
        for v in range {
            let part = match self.get_mut(v) {
                None => return,
                Some(x) => x,
            };
            part.ui(ui, state)
        }
    }
}

impl<T, Input: Clone, Comp: Component<T, Input = Input>, const N: usize> MultiComponent<T>
    for [Comp; N]
{
    fn len(&self) -> usize {
        N
    }
    fn process_part(&mut self, range: Range<usize>, state: &mut T) {
        for v in range {
            let part = self.get_mut(v);
            let part = match part {
                Some(x) => x,
                None => return,
            };
            part.process(state);
        }
    }

    fn render_part(&self, range: Range<usize>, state: &T) {
        for v in range {
            let part = self.get(v);
            let part = match part {
                Some(x) => x,
                None => return,
            };
            let state = state;
            part.render(state);
        }
    }
    fn ui_part(&mut self, range: Range<usize>, ui: &mut macroquad::ui::Ui, state: &mut T) {
        for v in range {
            let part = match self.get_mut(v) {
                None => return,
                Some(x) => x,
            };
            part.ui(ui, state)
        }
    }
}

pub struct PartRender<
    'a,
    State: 'a + Clone,
    Comp: MultiComponent<State>,
    Decider: Fn(usize, &State) -> Range<usize>,
> {
    child: Comp,
    _state: PhantomData<&'a State>,
    decider: Decider,
}

impl<
        'a,
        State: 'a + Clone,
        Comp: MultiComponent<State>,
        Decider: Fn(usize, &State) -> Range<usize>,
    > PartRender<'a, State, Comp, Decider>
{
    pub fn new(child: Comp, decider: Decider) -> Self {
        Self {
            child,
            decider,
            _state: PhantomData,
        }
    }
}
impl<
        'a,
        State: 'a + Clone,
        Comp: MultiComponent<State>,
        Decider: Fn(usize, &State) -> Range<usize>,
    > Component<State> for PartRender<'a, State, Comp, Decider>
{
    type Input = (Comp, Decider);

    fn instantiate((child, decider): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self::new(child, decider)
    }

    fn process(&mut self, state: &mut State) {
        let len = self.child.len();
        let range = (self.decider)(len, state);
        self.child.process_part(range, state)
    }

    fn render(&self, props: &State) {
        let len = self.child.len();
        let range = (self.decider)(len, props);
        self.child.render_part(range, props)
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        let len = self.child.len();
        let range = (self.decider)(len, state);
        self.child.ui_part(range, ui, state)
    }
}
