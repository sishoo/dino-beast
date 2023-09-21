use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::prelude::*;

const NUM_DINOS: usize = 1;
const NUM_FOOD: usize = 1;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 800.0;
const STEP: f32 = 0.5;

struct MainState {
    dino_x: Vec<f32>,
    dino_y: Vec<f32>,
    dino_dist: Vec<f32>,
    food_x: Vec<f32>,
    food_y: Vec<f32>,
    closest_food: Vec<usize>
}

impl MainState {
    fn new() -> GameResult<Self> {
        let mut rng = rand::thread_rng();
        let mut dino_x: Vec<f32> = Vec::with_capacity(NUM_DINOS);
        let mut dino_y: Vec<f32> = Vec::with_capacity(NUM_DINOS);
        let mut food_x: Vec<f32> = Vec::with_capacity(NUM_FOOD);
        let mut food_y: Vec<f32> = Vec::with_capacity(NUM_FOOD);
        let mut dino_dist: Vec<f32> = Vec::with_capacity(NUM_DINOS);
        let mut closest_food: Vec<usize> = Vec::with_capacity(NUM_DINOS);

        for _ in 0..NUM_DINOS {
            dino_x.push(rng.gen_range(0.0..SCREEN_WIDTH));
            dino_y.push(rng.gen_range(0.0..SCREEN_HEIGHT));
        }
        for _ in 0..NUM_FOOD {
            food_x.push(rng.gen_range(0.0..SCREEN_WIDTH));
            food_y.push(rng.gen_range(0.0..SCREEN_HEIGHT));
        }

        for dino in 0..NUM_DINOS {
            let dino_x = dino_x[dino];
            let dino_y = dino_y[dino];
            let mut min_dist = f32::MAX;
            let mut mindex: usize = 0;
            for food in 0..NUM_FOOD {
                let dist = dist_of(dino_x, dino_y, food_x[food], food_y[food]);
                if dist < min_dist {
                    min_dist = dist;
                    mindex = food;
                }
            }
            dino_dist.push(min_dist);
            closest_food.push(mindex);
        }

        Ok(Self {
            dino_x,
            dino_y,
            dino_dist,
            food_x,
            food_y,
            closest_food
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // think back to circle cut in half thing
        for dino in 0..NUM_DINOS {
            let mut dino_x = self.dino_x[dino];
            let mut dino_y = self.dino_y[dino];
            let mut current_dist = self.dino_dist[dino];
            let food_index = self.closest_food[dino];
            let food_x = self.food_x[food_index];
            let food_y = self.food_y[food_index];
            
            self.dino_x[dino] = dino_x;
            self.dino_y[dino] = dino_y;
            self.dino_dist[dino] = current_dist;
        }
        println!("fps {:?}", ctx.time.fps());
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

fn dist_of(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

fn main() -> GameResult {
    assert!(NUM_DINOS != 0);
    assert!(NUM_FOOD != 0);
    let cb = ggez::ContextBuilder::new("Dino Beast", "Mac")
        .window_setup(ggez::conf::WindowSetup::default().title("Dino Beast"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
