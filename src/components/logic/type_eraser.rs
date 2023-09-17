use crate::{components::Context, Component};

/// Can be used to erase some of the types from the final scene tree.
///
/// Can improve compile time performance at the cost of some runtime performance.
pub struct Eraser<RenderState, ProcessState, Input> {
    child: Box<dyn Component<RenderState, ProcessState, Input = Input>>,
}
impl<RenderState: Clone, ProcessState, Input> Component<RenderState, ProcessState>
    for Eraser<RenderState, ProcessState, Input>
{
    type Input = Box<dyn Component<RenderState, ProcessState, Input = Input>>;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { child: input }
    }

    fn process(&mut self, context: &Context, state: ProcessState) -> ProcessState {
        self.child.process(context, state)
    }

    fn render(&self, context: &Context, props: RenderState) {
        self.child.render(context, props)
    }
    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: ProcessState,
    ) -> ProcessState {
        self.child.ui(context, ui, state)
    }
}

impl<RenderState: Clone, ProcessState, Input> Eraser<RenderState, ProcessState, Input> {
    pub fn new<Comp: Component<RenderState, ProcessState, Input = Input> + 'static>(
        child: Comp,
    ) -> Self {
        Self::instantiate(Box::new(child))
    }
}
