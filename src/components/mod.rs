pub mod animation;
pub mod asyncs;
mod context;
pub mod data;
pub mod events;
pub mod logic;
pub mod render;
#[cfg(feature = "macroquad-tiled")]
pub mod tiled;
pub mod ui;

pub use context::Context;

use self::logic::Eraser;

/// The trait that the scene and state management is built on top off.
///
/// Every node in the scene tree implements this trait
pub trait Component<RenderState: Clone, ProcessState> {
    /// The type that the component needs to get in order to be instantiated
    type Input;
    /// Instantiates the component
    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized;
    /// Runs once every frame, allows the state to be mutated
    #[allow(unused_variables)]
    fn process(&mut self, context: &Context, state: ProcessState) -> ProcessState {
        state
    }
    /// Runs once every frame to render the current frame
    #[allow(unused_variables)]
    fn render(&self, context: &Context, props: RenderState) {}

    /// Runs once every frame to render the UI and update the state
    #[allow(unused_variables)]
    fn ui(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: ProcessState,
    ) -> ProcessState {
        state
    }

    /// Erases the types a bit, may improve compile time performance at the cost of runtime performance
    fn boxed(self) -> Eraser<RenderState, ProcessState, Self::Input>
    where
        Self: Sized + 'static,
    {
        Eraser::new(self)
    }
}

/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<RenderState: Clone, ProcessState, Comp: Component<RenderState, ProcessState>>
    Component<RenderState, ProcessState> for (Comp,)
{
    fn process(&mut self, context: &Context, state: ProcessState) -> ProcessState {
        self.0.process(context, state)
    }

    fn render(&self, context: &Context, props: RenderState) {
        self.0.render(context, props)
    }

    type Input = Comp::Input;

    fn instantiate(input: Self::Input) -> Self {
        (Comp::instantiate(input),)
    }

    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: ProcessState,
    ) -> ProcessState {
        self.0.ui(context, ui, state)
    }
}

/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<
        RenderState: Clone,
        ProcessState,
        Comp: Component<RenderState, ProcessState>,
        Comp2: Component<RenderState, ProcessState>,
    > Component<RenderState, ProcessState> for (Comp, Comp2)
{
    fn process(&mut self, context: &Context, state: ProcessState) -> ProcessState {
        let state = self.0.process(context, state);
        self.1.process(context, state)
    }

    fn render(&self, context: &Context, props: RenderState) {
        self.0.render(context, props.clone());
        self.1.render(context, props)
    }

    type Input = (Comp::Input, Comp2::Input);

    fn instantiate(input: Self::Input) -> Self {
        (Comp::instantiate(input.0), Comp2::instantiate(input.1))
    }

    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: ProcessState,
    ) -> ProcessState {
        let state = self.0.ui(context, ui, state);
        self.1.ui(context, ui, state)
    }
}

/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<
        RenderState: Clone,
        ProcessState,
        Comp: Component<RenderState, ProcessState>,
        Comp2: Component<RenderState, ProcessState>,
        Comp3: Component<RenderState, ProcessState>,
    > Component<RenderState, ProcessState> for (Comp, Comp2, Comp3)
{
    fn process(&mut self, context: &Context, state: ProcessState) -> ProcessState {
        let state = self.0.process(context, state);
        let state = self.1.process(context, state);
        self.2.process(context, state)
    }

    fn render(&self, context: &Context, props: RenderState) {
        self.0.render(context, props.clone());
        self.1.render(context, props.clone());
        self.2.render(context, props)
    }

    type Input = (Comp::Input, Comp2::Input, Comp3::Input);

    fn instantiate(input: Self::Input) -> Self {
        (
            Comp::instantiate(input.0),
            Comp2::instantiate(input.1),
            Comp3::instantiate(input.2),
        )
    }
    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: ProcessState,
    ) -> ProcessState {
        let state = self.0.ui(context, ui, state);
        let state = self.1.ui(context, ui, state);
        self.2.ui(context, ui, state)
    }
}

/// Here to help combine multiple components into 1
///
/// Will run process and render on each child in their respective order
impl<
        RenderState: Clone,
        ProcessState,
        Comp: Component<RenderState, ProcessState>,
        Comp2: Component<RenderState, ProcessState>,
        Comp3: Component<RenderState, ProcessState>,
        Comp4: Component<RenderState, ProcessState>,
    > Component<RenderState, ProcessState> for (Comp, Comp2, Comp3, Comp4)
{
    fn process(&mut self, context: &Context, state: ProcessState) -> ProcessState {
        let state = self.0.process(context, state);
        let state = self.1.process(context, state);
        let state = self.2.process(context, state);
        self.3.process(context, state)
    }

    fn render(&self, context: &Context, props: RenderState) {
        self.0.render(context, props.clone());
        self.1.render(context, props.clone());
        self.2.render(context, props.clone());
        self.3.render(context, props)
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
    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: ProcessState,
    ) -> ProcessState {
        let state = self.0.ui(context, ui, state);
        let state = self.1.ui(context, ui, state);
        let state = self.2.ui(context, ui, state);
        self.3.ui(context, ui, state)
    }
}
