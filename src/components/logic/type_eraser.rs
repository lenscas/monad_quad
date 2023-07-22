use crate::Component;

/// Can be used to erase some of the types from the final scene tree.
///
/// Can improve compile time performance at the cost of some runtime performance.
pub struct Eraser<State, Input> {
    child: Box<dyn Component<State, Input = Input>>,
}
impl<State, Input> Component<State> for Eraser<State, Input> {
    type Input = Box<dyn Component<State, Input = Input>>;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { child: input }
    }

    fn process(&mut self, state: &mut State) {
        self.child.process(state)
    }

    fn render(&self, props: &State) {
        self.child.render(props)
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        self.child.ui(ui, state)
    }
}

impl<State, Input> Eraser<State, Input> {
    pub fn new<Comp: Component<State, Input = Input> + 'static>(child: Comp) -> Self {
        Self::instantiate(Box::new(child))
    }
}
