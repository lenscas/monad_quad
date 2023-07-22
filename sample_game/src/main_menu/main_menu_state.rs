#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OnScreen {
    MainMenu,
    Settings,
}

#[derive(Clone, Debug)]
pub enum SwitchingTo {
    None,
    Switch {
        from: OnScreen,
        to: OnScreen,
        full_progress: f32,
        at: f32,
    },
}

impl SwitchingTo {
    pub fn get_progress_from(&self, from: OnScreen) -> Option<f32> {
        match self {
            SwitchingTo::Switch {
                from: from_2,
                full_progress,
                ..
            } if from == *from_2 => Some(*full_progress),
            SwitchingTo::Switch {
                to, full_progress, ..
            } if from == *to => Some(-full_progress),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainMenuButtonSelected {
    StartGame,
    Settings,
}

#[derive(Clone, Debug)]
pub struct MainMenuProperties {
    pub started_game: bool,
    pub switching: SwitchingTo,
    pub selected_button: Option<MainMenuButtonSelected>,
    pub on_screen: OnScreen,
}
impl MainMenuProperties {
    pub fn new() -> Self {
        Self {
            started_game: false,
            switching: SwitchingTo::None,
            selected_button: None,
            on_screen: OnScreen::MainMenu,
        }
    }
}

impl Default for MainMenuProperties {
    fn default() -> Self {
        Self::new()
    }
}
