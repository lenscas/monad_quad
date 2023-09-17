use std::path::PathBuf;

use macroquad::{
    prelude::{error, vec2, warn, Vec2},
    window::request_new_screen_size,
};

fn config_dir() -> Option<PathBuf> {
    let x = directories::ProjectDirs::from("", "monad_quad", "sample_game")
        .map(|v| v.config_dir().to_path_buf());
    if let Some(x) = &x {
        if !x.exists() {
            if let Err(x) = std::fs::create_dir(x) {
                error!("Could not create settings directory!");
                error!("Error: {}", x);
            }
        }
    }
    x
}
fn get_settings_file() -> Option<PathBuf> {
    config_dir().map(|mut v| {
        v.push("settings.json");
        v
    })
}

pub const RESOLUTIONS: [(Vec2, &str); 5] = [
    (vec2(1280., 720.), "1280 x 720 (720p)"),
    (vec2(1920., 1080.), "1920 x 1080 (1080p)"),
    (vec2(2560., 1440.), "2560 x 1440 (1440p)"),
    (vec2(3840., 2160.), "3840 x 2160 (4K UHD/2160p)"),
    (vec2(7680., 4320.), "7680 x 4320 (8K FUHD/4320p)"),
];

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct Settings {
    pub is_fullscreen: bool,
    pub selected_size: usize,
}
impl Settings {
    pub async fn read_from_settings_or_default() -> Self {
        let config_dir = config_dir();
        let settings = match config_dir {
            None => Settings::default(),
            Some(mut x) => {
                x.push("settings.json");
                match macroquad::file::load_string(&x.to_string_lossy()).await {
                    Err(x) => {
                        warn!("Could not load config file.");
                        warn!("Error: {}", x);
                        warn!("Using default settings instead");
                        Settings::default()
                    }
                    Ok(x) => serde_json::from_str(&x).unwrap_or_else(|v| {
                        error!("Failed to deserialize config file");
                        error!("Error: {}", v);
                        error!("using default settings instead");
                        Default::default()
                    }),
                }
            }
        };
        settings.apply_current_settings();
        settings
    }
    fn set_screen_size(&self) {
        let (selected_size, _) = RESOLUTIONS[self.selected_size];
        request_new_screen_size(selected_size.x, selected_size.y)
    }
    pub fn apply_current_settings(&self) {
        //set_fullscreen(self.is_fullscreen);
        self.set_screen_size();
    }
    pub fn apply_new_settings(&mut self, new: Self) {
        let mut save_file = false;
        if self.is_fullscreen != new.is_fullscreen {
            //set_fullscreen(new.is_fullscreen);
            self.is_fullscreen = new.is_fullscreen;
            save_file = true;
        }
        if self.selected_size != new.selected_size {
            self.selected_size = new.selected_size;
            self.set_screen_size();
            save_file = true
        }

        if !save_file {
            return;
        }
        match get_settings_file() {
            None => {
                warn!("Could not get settings file location");
                warn!("Settings will not be saved!");
            }
            Some(x) => {
                if let Err(x) = std::fs::write(x, serde_json::to_string_pretty(&self).unwrap()) {
                    error!("Could not save settings data.");
                    error!("Error: {}", x);
                }
            }
        }
    }
}
