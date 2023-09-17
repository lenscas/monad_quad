mod main_menu;
mod nodes;
mod settings;

use macroquad::prelude::*;

use main_menu::{main_menu2, MainMenuProperties, SwitchingTo};
use monad_quad::components::{
    events::{Event, KeyDown},
    logic::{Choice, Comp, OnlyRenderWith, StateFull, StaticValue},
    render::{Background, Text, TextProperties, Viewport},
};
use nodes::{
    CoinProperties, Coins, ControlProps, Enemies, EnemyProperties, ItemRendererProperties, Player,
    PlayerProps, ScoreDisplay, ScoreDisplayProperties,
};
use settings::Settings;

struct MainState {
    started_game: bool,
    player_state: PlayerProps,
    coins: Vec<Vec2>,
    coin_size: f32,
    coin_speed: f32,
    coin_spawn_chance: f32,
    enemy_size: f32,
    enemy_spawn_chance: f32,
    enemies: Vec<Vec2>,
    enemy_speed: f32,
    paused: bool,
    switching: SwitchingTo,
    settings: Settings,
}
impl MainState {
    pub fn new_with_settings(settings: Settings) -> Self {
        MainState {
            switching: SwitchingTo::OnScreen(main_menu::OnScreen::MainMenu),
            started_game: false,

            paused: false,
            player_state: PlayerProps {
                lives: 3,
                size: vec2(10., 15.),
                score: 0,
                location: vec2(screen_width() / 2., screen_height() / 2.),
                control: ControlProps {
                    left: KeyCode::Left,
                    right: KeyCode::Right,
                    up: KeyCode::Up,
                    down: KeyCode::Down,
                    speed: 100.,
                    dir: vec2(0., 0.),
                },
            },

            coin_size: 30.,
            coin_spawn_chance: 0.02,
            coin_speed: 30.,
            coins: Vec::new(),

            enemy_size: 25.,
            enemy_spawn_chance: 0.025,
            enemy_speed: 35.,
            enemies: Vec::new(),
            settings,
        }
    }
    pub async fn new() -> Self {
        let settings = Settings::read_from_settings_or_default().await;
        Self::new_with_settings(settings)
    }
    pub fn set_settings(&mut self, settings: Settings) {
        self.settings.apply_new_settings(settings)
    }
}
#[macroquad::main("Sample game")]
async fn main() {
    let state = MainState::new().await;

    let mut state = StateFull::new_from(state);

    let scene_tree = Viewport::new::<MainState>(
        vec2(1920., 1080.),
        (
            StaticValue::new(BLACK, Background),
            Choice::new(
                |x: &MainState| !x.started_game,
                Comp::map_in(|v: &MainState| MainMenuProperties {
                    switching: v.switching.to_owned(),
                    selected_button: None,
                    started_game: v.started_game,
                    settings: v.settings,
                })
                .map_out_for(
                    |props, state| {
                        state.started_game = props.started_game;
                        state.switching = props.switching;
                        state.set_settings(props.settings);
                    },
                    main_menu2(),
                ),
                Choice::new(
                    |x| x.player_state.lives > 0,
                    (
                        Event::new(
                            |_: &MainState| is_key_pressed(KeyCode::P),
                            |v, state| {
                                if v {
                                    state.paused = !state.paused
                                }
                            },
                        ),
                        OnlyRenderWith::new(
                            |v: &MainState| !v.paused,
                            (
                                Comp::<_, Player>::map_in(|state: &MainState| {
                                    state.player_state.to_owned()
                                })
                                .map_out(|new_state, state| state.player_state = new_state),
                                Comp::<_, Coins>::map_in(|state: &MainState| {
                                    ItemRendererProperties {
                                        items: state.coins.to_owned(),
                                        extra_data: CoinProperties {
                                            coin_size: state.coin_size,
                                            coin_speed: state.coin_speed,
                                            spawn_chance: state.coin_spawn_chance,
                                            player_loc: state.player_state.location,
                                            player_size: state.player_state.size,
                                            touched_coins: 0,
                                            coins_let_through: 0,
                                        },
                                    }
                                })
                                .map_out(|new_state, state| {
                                    state.coins = new_state.items;
                                    state.player_state.score += (new_state.extra_data.touched_coins
                                        * 10)
                                        - (new_state.extra_data.coins_let_through * 15)
                                }),
                                Comp::<_, Enemies>::map_in(|state: &MainState| {
                                    ItemRendererProperties {
                                        items: state.enemies.to_owned(),
                                        extra_data: EnemyProperties {
                                            enemies_size: state.enemy_size,
                                            enemies_color: RED,
                                            enemies_speed: state.enemy_speed,
                                            enemies_chance: state.enemy_spawn_chance,
                                            player_loc: state.player_state.location,
                                            player_size: state.player_state.size,
                                            got_hit: false,
                                            reached_the_end: 0,
                                        },
                                    }
                                })
                                .map_out(|new_state, state| {
                                    state.enemies = new_state.items;
                                    state.player_state.score +=
                                        new_state.extra_data.reached_the_end * 11;
                                    if new_state.extra_data.got_hit {
                                        state.player_state.lives -= 1;
                                    }
                                }),
                                Comp::<_, ScoreDisplay>::map_in(|state: &MainState| {
                                    ScoreDisplayProperties {
                                        score: state.player_state.score,
                                        location: vec2(10., 10.),
                                        font_size: 14.,
                                        color: GREEN,
                                        lives: state.player_state.lives,
                                        lives_size: 14.,
                                        lives_location: vec2(10., 15.),
                                    }
                                })
                                .map_out(|_, _| {}),
                            ),
                            StaticValue::new(
                                TextProperties {
                                    text: "Paused".to_string(),
                                    location: vec2(100., 100.),
                                    font_size: 20.,
                                    color: GREEN,
                                },
                                Text,
                            ),
                        ),
                    ),
                    (
                        KeyDown::new(
                            |_: &MainState| KeyCode::Enter,
                            |v, state| {
                                if v {
                                    let mut new_state =
                                        MainState::new_with_settings(state.settings);
                                    new_state.started_game = true;
                                    *state = new_state;
                                }
                            },
                        ),
                        StaticValue::new(
                            TextProperties {
                                text: "Game Over".to_string(),
                                location: vec2(10., 20.),
                                font_size: 40.,
                                color: RED,
                            },
                            Text,
                        ),
                    ),
                ),
            ),
        ),
    );
    state.render(scene_tree).await;
}
