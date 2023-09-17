mod settings_menu_state;

use macroquad::hash;
pub use settings_menu_state::SettingsMenuProperties;

use monad_quad::{
    components::{
        logic::Comp,
        ui::{Button, CheckBox, ComboBox, Window},
    },
    Component,
};

use super::{MainMenuProperties, OnScreen, SwitchingTo};

pub fn draw_settings_menu(
) -> impl for<'a> Component<&'a MainMenuProperties, &'a mut MainMenuProperties> {
    Comp::map_in(MainMenuProperties::to_settings_menu_properties).map_out_for(
        MainMenuProperties::merge_from_settings_menu_properties,
        Comp::map_in(SettingsMenuProperties::to_top_window_props).map_out_for(
            SettingsMenuProperties::merge_from_top_window_props,
            Window::new(
                hash!(),
                (
                    Comp::<_, _>::map_in(SettingsMenuProperties::to_back_menu_props).map_out_for(
                        SettingsMenuProperties::merge_back_menu_props,
                        Button::instantiate(|v| {
                            v.extra_data = SwitchingTo::Switch {
                                from: OnScreen::Settings,
                                to: OnScreen::MainMenu,
                                full_progress: 0.0,
                                at: 0.,
                            };
                        }),
                    ),
                    Comp::<_, ComboBox>::map_in(SettingsMenuProperties::to_screen_size_props)
                        .map_out(SettingsMenuProperties::merge_screen_size_select),
                    Comp::<_, CheckBox>::map_in(SettingsMenuProperties::to_full_screen_button)
                        .map_out(SettingsMenuProperties::merge_full_screen_button),
                ),
            ),
        ),
    )
}
