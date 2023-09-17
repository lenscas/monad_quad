use macroquad::{
    prelude::vec2,
    ui::root_ui,
    window::{next_frame, screen_height, screen_width},
};

use crate::{components::Context, Component};

/// This manages the state and controls the game loop
pub struct StateFull<T> {
    state: T,
}
impl<T> StateFull<T> {
    /// creates a new instance with the given state
    pub fn new_from(state: T) -> Self {
        Self { state }
    }
    /// starts the game loop
    pub async fn render<'b, Comp: for<'a> Component<&'a T, &'a mut T>>(
        &'b mut self,
        mut component: Comp,
    ) {
        let mut state = &mut self.state;
        loop {
            let context = Context::new(vec2(screen_width(), screen_height()));
            {
                state = component.process(&context, state);
            }
            {
                component.render(&context, state);
            }

            {
                let mut ui = root_ui();
                state = component.ui(&context, &mut ui, state);
            }

            next_frame().await
        }
    }
}
impl<T: Default> StateFull<T> {
    /// creates a new instance with the default value for the state
    pub fn new() -> Self {
        Self::new_from(T::default())
    }
}

impl<T: Default> Default for StateFull<T> {
    fn default() -> Self {
        Self::new()
    }
}
