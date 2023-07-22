use std::marker::PhantomData;

use crate::components::Component;
/// Allows you to turn off one of the 2 possible scene tree branches based on the current state
/// basically an `if condition { true_child } else { false_child }`
pub struct Choice<
    State,
    Choose: Fn(&State) -> bool,
    TrueChild: Component<State>,
    FalseChild: Component<State>,
> {
    _state: PhantomData<State>,
    chooser: Choose,
    true_child: TrueChild,
    false_child: FalseChild,
}

impl<
        State,
        Choose: Fn(&State) -> bool,
        TrueChild: Component<State>,
        FalseChild: Component<State>,
    > Choice<State, Choose, TrueChild, FalseChild>
{
    pub fn new(choice: Choose, true_child: TrueChild, false_child: FalseChild) -> Self {
        Self::instantiate((choice, true_child, false_child))
    }
}

impl<
        State,
        Choose: Fn(&State) -> bool,
        TrueChild: Component<State>,
        FalseChild: Component<State>,
    > Component<State> for Choice<State, Choose, TrueChild, FalseChild>
{
    type Input = (Choose, TrueChild, FalseChild);

    fn instantiate((chooser, true_child, false_child): Self::Input) -> Self {
        Self {
            _state: PhantomData,
            chooser,
            true_child,
            false_child,
        }
    }

    fn process(&mut self, state: &mut State) {
        if (self.chooser)(state) {
            self.true_child.process(state)
        } else {
            self.false_child.process(state);
        }
    }

    fn render(&self, props: &State) {
        if (self.chooser)(props) {
            self.true_child.render(props)
        } else {
            self.false_child.render(props)
        }
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut State) {
        if (self.chooser)(state) {
            self.true_child.ui(ui, state)
        } else {
            self.false_child.ui(ui, state)
        }
    }
}
