use macroquad::{
    prelude::{vec2, Color, Rect, Vec2},
    rand::RandomRange,
    time::get_frame_time,
    window::{screen_height, screen_width},
};
use monad_quad::components::{
    render::{Rectangle, RectangleProps},
    Component,
};

use super::{
    coins::{Spawner, SpawnerConfig},
    ChildProperties, ItemRenderer, ItemRendererProperties,
};

#[derive(Debug, Clone)]
pub struct EnemyProperties {
    pub enemies_size: f32,
    pub enemies_color: Color,
    pub enemies_speed: f32,
    pub enemies_chance: f32,
    pub player_loc: Vec2,
    pub player_size: Vec2,
    pub got_hit: bool,
    pub reached_the_end: i64,
}

pub struct SingleEnemyRenderer {
    child: Rectangle,
}

impl Component<ChildProperties<EnemyProperties>> for SingleEnemyRenderer {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self { child: Rectangle }
    }

    fn process(&mut self, state: &mut ChildProperties<EnemyProperties>) {
        let frame_time = get_frame_time();
        let player_rec = Rect::new(
            state.extra_data.player_loc.x,
            state.extra_data.player_loc.y,
            state.extra_data.player_size.x,
            state.extra_data.player_size.y,
        );
        state.location.y -= frame_time * state.extra_data.enemies_speed;
        let enemy_rec = Rect::new(
            state.location.x,
            state.location.y,
            state.extra_data.enemies_size,
            state.extra_data.enemies_size,
        );
        if player_rec.overlaps(&enemy_rec) {
            state.extra_data.got_hit = true;
            state.destroyed = true;
        } else if state.location.y < 0. {
            state.extra_data.reached_the_end += 1;
            state.destroyed = true;
        }
    }

    fn render(&self, props: &ChildProperties<EnemyProperties>) {
        self.child.render(&RectangleProps {
            size: vec2(props.extra_data.enemies_size, props.extra_data.enemies_size),
            color: props.extra_data.enemies_color,
            location: props.location,
        })
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut ChildProperties<EnemyProperties>) {
        self.child.ui(
            ui,
            &mut RectangleProps {
                size: vec2(state.extra_data.enemies_size, state.extra_data.enemies_size),
                color: state.extra_data.enemies_color,
                location: state.location,
            },
        )
    }
}

type EnemySpawner = Spawner<Vec2, EnemyProperties, fn(&mut SpawnerConfig<EnemyProperties, Vec2>)>;

pub struct Enemies {
    spawner: EnemySpawner,
    renderer: ItemRenderer<EnemyProperties, SingleEnemyRenderer>,
}
impl Component<ItemRendererProperties<EnemyProperties>> for Enemies {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        fn on_reached_time(props: &mut SpawnerConfig<EnemyProperties, Vec2>) {
            let window_height = screen_height();
            if f32::gen_range(0., 1.) < props.extra_data.enemies_chance {
                props.spawned_items.push(vec2(
                    f32::gen_range(0., screen_width() + props.extra_data.enemies_size),
                    window_height, //f32::gen_range(0., screen_height() - state.coin_size),
                ))
            }
        }
        Self {
            spawner: Spawner::new(on_reached_time),
            renderer: ItemRenderer::instantiate(SingleEnemyRenderer::instantiate(())),
        }
    }

    fn process(&mut self, state: &mut ItemRendererProperties<EnemyProperties>) {
        let mut config = SpawnerConfig {
            max_time: 1.0 / 60.,
            size: vec2(state.extra_data.enemies_size, state.extra_data.enemies_size),
            extra_data: state.extra_data.to_owned(),
            spawned_items: state.items.to_owned(),
        };
        self.spawner.process(&mut config);
        state.extra_data = config.extra_data;
        state.items = config.spawned_items;
        self.renderer.process(state);
    }

    fn render(&self, props: &ItemRendererProperties<EnemyProperties>) {
        let config = SpawnerConfig {
            max_time: 1.0 / 60.,
            size: vec2(props.extra_data.enemies_size, props.extra_data.enemies_size),
            extra_data: props.extra_data.to_owned(),
            spawned_items: props.items.to_owned(),
        };
        self.spawner.render(&config);
        self.renderer.render(props);
    }
    fn ui(
        &mut self,
        ui: &mut macroquad::ui::Ui,
        state: &mut ItemRendererProperties<EnemyProperties>,
    ) {
        let mut config = SpawnerConfig {
            max_time: 1.0 / 60.,
            size: vec2(state.extra_data.enemies_size, state.extra_data.enemies_size),
            extra_data: state.extra_data.to_owned(),
            spawned_items: state.items.to_owned(),
        };
        self.spawner.ui(ui, &mut config);
        self.renderer.ui(ui, state);
    }
}
