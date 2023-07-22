use std::marker::PhantomData;

use macroquad::{
    prelude::{vec2, Rect, Vec2, YELLOW},
    rand::RandomRange,
    time::get_frame_time,
    window::{screen_height, screen_width},
};
use monad_quad::components::{
    events::VariableTimer,
    render::{Rectangle, RectangleProps},
    Component,
};

pub struct SpawnerConfig<ExtraData, T> {
    pub max_time: f32,
    pub size: Vec2,
    pub extra_data: ExtraData,
    pub spawned_items: Vec<T>,
}

type ChildTimer<ExtraData, Item, Spawn> = VariableTimer<
    SpawnerConfig<ExtraData, Item>,
    fn(&SpawnerConfig<ExtraData, Item>) -> f32,
    Spawn,
>;

pub struct Spawner<Item, ExtraData, Spawn: Fn(&mut SpawnerConfig<ExtraData, Item>)> {
    timer: ChildTimer<ExtraData, Item, Spawn>,
    _spawner: PhantomData<Spawn>,
}
impl<Item, ExtraData, Spawn: Fn(&mut SpawnerConfig<ExtraData, Item>)>
    Spawner<Item, ExtraData, Spawn>
{
    pub fn new(spawn: Spawn) -> Self {
        Self::instantiate(spawn)
    }
}

impl<Item, ExtraData, Spawn: Fn(&mut SpawnerConfig<ExtraData, Item>)>
    Component<SpawnerConfig<ExtraData, Item>> for Spawner<Item, ExtraData, Spawn>
{
    type Input = Spawn;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        fn get_max_time<ExtraData, Item>(state: &SpawnerConfig<ExtraData, Item>) -> f32 {
            state.max_time
        }
        Self {
            timer: VariableTimer::new(get_max_time, input),
            _spawner: PhantomData,
        }
    }

    fn process(&mut self, state: &mut SpawnerConfig<ExtraData, Item>) {
        self.timer.process(state)
    }

    fn render(&self, props: &SpawnerConfig<ExtraData, Item>) {
        self.timer.render(props)
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut SpawnerConfig<ExtraData, Item>) {
        self.timer.ui(ui, state)
    }
}

pub struct ItemRendererProperties<T: Clone> {
    pub items: Vec<Vec2>,
    pub extra_data: T,
}

pub struct ChildProperties<T> {
    pub extra_data: T,
    pub location: Vec2,
    pub destroyed: bool,
}

pub struct ItemRenderer<ExtraData: Clone, Child: Component<ChildProperties<ExtraData>>> {
    _extra_data: PhantomData<ExtraData>,
    child: Child,
}

impl<ExtraData: Clone, Child: Component<ChildProperties<ExtraData>>>
    Component<ItemRendererProperties<ExtraData>> for ItemRenderer<ExtraData, Child>
{
    type Input = Child;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            _extra_data: PhantomData,
            child: input,
        }
    }

    fn process(&mut self, state: &mut ItemRendererProperties<ExtraData>) {
        state.items = state
            .items
            .drain(0..)
            .filter_map(|mut v| {
                let mut child_state = ChildProperties {
                    destroyed: false,
                    extra_data: state.extra_data.to_owned(),
                    location: v,
                };
                self.child.process(&mut child_state);
                state.extra_data = child_state.extra_data;
                v = child_state.location;
                if child_state.destroyed {
                    None
                } else {
                    Some(v)
                }
            })
            .collect()
    }

    fn render(&self, props: &ItemRendererProperties<ExtraData>) {
        for v in &props.items {
            let child_state = ChildProperties {
                destroyed: false,
                extra_data: props.extra_data.to_owned(),
                location: *v,
            };
            self.child.render(&child_state);
        }
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut ItemRendererProperties<ExtraData>) {
        for v in &state.items {
            let mut child_state = ChildProperties {
                destroyed: false,
                extra_data: state.extra_data.to_owned(),
                location: *v,
            };
            self.child.ui(ui, &mut child_state);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoinProperties {
    pub spawn_chance: f32,
    pub coin_size: f32,
    pub coin_speed: f32,
    pub player_loc: Vec2,
    pub player_size: Vec2,
    pub touched_coins: i64,
    pub coins_let_through: i64,
}

pub struct SingleCoinRenderer {
    renderer: Rectangle,
}
impl Component<ChildProperties<CoinProperties>> for SingleCoinRenderer {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            renderer: Rectangle,
        }
    }

    fn process(&mut self, state: &mut ChildProperties<CoinProperties>) {
        let mut coin_loc = state.location;
        let frame_time = get_frame_time();
        let window_height = screen_height();
        let player_rec = Rect::new(
            state.extra_data.player_loc.x,
            state.extra_data.player_loc.y,
            state.extra_data.player_size.x,
            state.extra_data.player_size.y,
        );
        coin_loc.y += frame_time * state.extra_data.coin_speed;
        state.location.y = coin_loc.y;
        let coin_rec = Rect::new(
            coin_loc.x,
            coin_loc.y,
            state.extra_data.coin_size,
            state.extra_data.coin_size,
        );
        if player_rec.overlaps(&coin_rec) {
            state.extra_data.touched_coins += 1;
            state.destroyed = true;
        } else if coin_loc.y > window_height {
            state.extra_data.coins_let_through += 1;
            state.destroyed = true;
        }
    }

    fn render(&self, props: &ChildProperties<CoinProperties>) {
        self.renderer.render(&RectangleProps {
            size: vec2(props.extra_data.coin_size, props.extra_data.coin_size),
            color: YELLOW,
            location: props.location,
        })
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut ChildProperties<CoinProperties>) {
        self.renderer.ui(
            ui,
            &mut RectangleProps {
                size: vec2(state.extra_data.coin_size, state.extra_data.coin_size),
                color: YELLOW,
                location: state.location,
            },
        )
    }
}

type CoinSpawner = Spawner<Vec2, CoinProperties, fn(&mut SpawnerConfig<CoinProperties, Vec2>)>;

pub struct Coins {
    spawner: CoinSpawner,
    renderer: ItemRenderer<CoinProperties, SingleCoinRenderer>,
}
impl Component<ItemRendererProperties<CoinProperties>> for Coins {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        fn on_reached_time(props: &mut SpawnerConfig<CoinProperties, Vec2>) {
            if f32::gen_range(0., 1.) < props.extra_data.spawn_chance {
                props.spawned_items.push(vec2(
                    f32::gen_range(0., screen_width() - props.extra_data.coin_size),
                    0., //f32::gen_range(0., screen_height() - state.coin_size),
                ))
            }
        }
        Self {
            spawner: Spawner::new(on_reached_time),
            renderer: ItemRenderer {
                _extra_data: PhantomData,
                child: SingleCoinRenderer {
                    renderer: Rectangle,
                },
            },
        }
    }

    fn process(&mut self, state: &mut ItemRendererProperties<CoinProperties>) {
        let mut spawner_config = SpawnerConfig {
            max_time: 1.0 / 60.,
            size: vec2(state.extra_data.coin_size, state.extra_data.coin_size),
            extra_data: state.extra_data.to_owned(),
            spawned_items: state.items.to_owned(),
        };
        self.spawner.process(&mut spawner_config);
        state.extra_data = spawner_config.extra_data;
        state.items = spawner_config.spawned_items;
        self.renderer.process(state);
    }

    fn render(&self, props: &ItemRendererProperties<CoinProperties>) {
        let spawner_config = SpawnerConfig {
            max_time: 1.0 / 60.,
            size: vec2(props.extra_data.coin_size, props.extra_data.coin_size),
            extra_data: props.extra_data.to_owned(),
            spawned_items: props.items.to_owned(),
        };
        self.spawner.render(&spawner_config);
        self.renderer.render(props);
    }

    fn ui(
        &mut self,
        ui: &mut macroquad::ui::Ui,
        state: &mut ItemRendererProperties<CoinProperties>,
    ) {
        let mut spawner_config = SpawnerConfig {
            max_time: 1.0 / 60.,
            size: vec2(state.extra_data.coin_size, state.extra_data.coin_size),
            extra_data: state.extra_data.to_owned(),
            spawned_items: state.items.to_owned(),
        };
        self.spawner.ui(ui, &mut spawner_config);
        self.renderer.ui(ui, state);
    }
}
