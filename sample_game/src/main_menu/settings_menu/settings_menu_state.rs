use macroquad::{
    prelude::vec2,
    window::{screen_height, screen_width},
};
use monad_quad::components::ui::{
    ButtonProperties, CheckboxProperties, ComboBoxProperties, WindowProperties,
};

use crate::{
    main_menu::{OnScreen, SwitchingTo},
    settings::{Settings, RESOLUTIONS},
};

#[derive(Clone)]
pub struct SettingsMenuProperties {
    pub settings: Settings,
    pub switch: SwitchingTo,
    pub has_back_button_selected: bool,
}

impl SettingsMenuProperties {
    pub fn to_top_window_props(&self) -> WindowProperties<Self> {
        let location = self.switch.get_location_of_window(OnScreen::Settings);
        WindowProperties {
            location,
            size: vec2(screen_width(), screen_height()),
            label: Some("Settings".into()),
            title_bar: true,
            extra_data: self.to_owned(),
            moveable: false,
            close_button: false,
        }
    }
    pub fn merge_from_top_window_props(props: WindowProperties<Self>, me: &mut Self) {
        *me = props.extra_data;
    }
    pub fn to_back_menu_props<'a>(&self) -> ButtonProperties<'a, SwitchingTo> {
        ButtonProperties {
            size: vec2(100., 50.),
            selected: self.has_back_button_selected,
            position: vec2(screen_width() / 2. - 50., screen_height() / 2. + 30.),
            extra_data: self.switch.clone(),
            content: "Settings!".into(),
        }
    }
    pub fn merge_back_menu_props(merge: ButtonProperties<SwitchingTo>, me: &mut Self) {
        me.switch = merge.extra_data;
    }
    pub fn to_full_screen_button(&self) -> CheckboxProperties {
        CheckboxProperties {
            state: self.settings.is_fullscreen,
            label: "Fullscreen".to_string(),
        }
    }
    pub fn merge_full_screen_button(a: CheckboxProperties, b: &mut Self) {
        b.settings.is_fullscreen = a.state;
    }
    pub fn to_screen_size_props(&self) -> ComboBoxProperties {
        let resolutions = RESOLUTIONS
            .into_iter()
            .map(|(_, v)| v.to_string())
            .collect();
        ComboBoxProperties {
            label: "Resolution".to_string(),
            variants: resolutions,
            chosen: Some(self.settings.selected_size),
        }
    }
    pub fn merge_screen_size_select(a: ComboBoxProperties, me: &mut Self) {
        let a = match a.chosen {
            None => return,
            Some(x) => x,
        };
        if a < RESOLUTIONS.len() && a != me.settings.selected_size {
            me.settings.selected_size = a;
        }
    }
}
