use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::prelude::*;

const NUM_DINOS: usize = 100;
const NUM_FOOD: usize = 100;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 800.0;

struct MainState {
    dino_x: [f32; NUM_DINOS],
    dino_y: [f32; NUM_DINOS],
    dino_dist: [f32; NUM_DINOS],
    food_x: [f32; NUM_FOOD],
    food_y: [f32; NUM_FOOD]
}

impl MainState {
    fn new() -> GameResult<Self> {
        let mut rng = rand::thread_rng();
        let mut dino_x = [0.0; NUM_DINOS];
        let mut dino_y = [0.0; NUM_DINOS];
        let mut food_x = [0.0; NUM_FOOD];
        let mut food_y = [0.0; NUM_FOOD];
        let mut dino_dist = [0.0; NUM_DINOS];

        dino_x.iter_mut().for_each(|x| *x = rng.gen_range(0.0..SCREEN_WIDTH));
        dino_y.iter_mut().for_each(|y| *y = rng.gen_range(0.0..SCREEN_HEIGHT));
        food_x.iter_mut().for_each(|x| *x = rng.gen_range(0.0..SCREEN_WIDTH));
        food_y.iter_mut().for_each(|y| *y = rng.gen_range(0.0..SCREEN_HEIGHT));

        for dino in 0..NUM_DINOS {
            let dino_x = dino_x[dino];
            let dino_y = dino_y[dino];
            let mut distances = Vec::with_capacity(NUM_DINOS);
            for food in 0..NUM_FOOD {
                distances.push(euclidian_dist_between(dino_x, dino_y, food_x[food], food_y[food]));
            }
            dino_dist[dino] = distances.iter().min();
        }

        Ok(Self {
            dino_x,
            dino_y,
            dino_dist,
            food_x,
            food_y
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        let dino_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            6.0,
            2.0,
            Color::GREEN,
        )?;

        let food_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            6.0,
            2.0,
            Color::RED,
        )?;

        for index in 0..NUM_DINOS {
            canvas.draw(&dino_mesh, Vec2::new(self.dino_x[index], self.dino_y[index]));
        }
        for index in 0..NUM_FOOD {
            canvas.draw(&food_mesh, Vec2::new(self.food_x[index], self.food_y[index]));
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn euclidian_dist_between(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

fn main() -> GameResult {
    assert!(NUM_DINOS == 0);
    assert!(NUM_FOOD == 0);
    let cb = ggez::ContextBuilder::new("Dino Beast", "Mac")
        .window_setup(ggez::conf::WindowSetup::default().title("Dino Beast"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}