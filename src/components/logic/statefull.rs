use macroquad::{ui::root_ui, window::next_frame};

use crate::Component;

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
    pub async fn render<Comp: Component<T>>(&mut self, mut component: Comp) {
        loop {
            component.render(&self.state);
            component.process(&mut self.state);
            {
                let mut ui = root_ui();
                component.ui(&mut ui, &mut self.state);
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
