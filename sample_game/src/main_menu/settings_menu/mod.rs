mod settings_menu_state;

use macroquad::hash;
use settings_menu_state::SettingsMenuProperties;

use monad_quad::{
    components::{logic::Never, ui::Window},
    Component,
};

pub fn draw_settings_menu() -> impl Component<SettingsMenuProperties> {
    Never
    // Window::new(hash!(),

    // )
}
