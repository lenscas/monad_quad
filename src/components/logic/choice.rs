use crate::components::{Component, Context};
/// Allows you to turn off one of the 2 possible scene tree branches based on the current state
/// basically an `if condition { true_child } else { false_child }`
pub struct Choice<Choose, TrueChild, FalseChild> {
    chooser: Choose,
    true_child: TrueChild,
    false_child: FalseChild,
}

impl<Choose, TrueChild, FalseChild> Choice<Choose, TrueChild, FalseChild> {
    pub fn new<ProcessState>(choice: Choose, true_child: TrueChild, false_child: FalseChild) -> Self
    where
        Choose: Fn(&ProcessState) -> bool,
        TrueChild: for<'a> Component<&'a ProcessState, &'a mut ProcessState>,
        FalseChild: for<'a> Component<&'a ProcessState, &'a mut ProcessState>,
    {
        Self::instantiate((choice, true_child, false_child))
    }
}

impl<
        State,
        Choose: Fn(&State) -> bool,
        TrueChild: for<'a> Component<&'a State, &'a mut State>,
        FalseChild: for<'a> Component<&'a State, &'a mut State>,
    > Component<&State, &mut State> for Choice<Choose, TrueChild, FalseChild>
{
    type Input = (Choose, TrueChild, FalseChild);

    fn instantiate((chooser, true_child, false_child): Self::Input) -> Self {
        Self {
            chooser,
            true_child,
            false_child,
        }
    }

    fn process<'c>(&mut self, context: &Context, state: &'c mut State) -> &'c mut State {
        if (self.chooser)(state) {
            self.true_child.process(context, state)
        } else {
            self.false_child.process(context, state)
        }
    }

    fn render(&self, context: &Context, props: &State) {
        if (self.chooser)(props) {
            self.true_child.render(context, props)
        } else {
            self.false_child.render(context, props)
        }
    }
    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut State,
    ) -> &'c mut State {
        if (self.chooser)(state) {
            self.true_child.ui(context, ui, state)
        } else {
            self.false_child.ui(context, ui, state)
        }
    }
}
