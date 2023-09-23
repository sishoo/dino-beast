use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use rand::prelude::*;

const NUM_DINOS: usize = 1;
const NUM_FOOD: usize = 1;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 800.0;
const STEP: f32 = 0.5;
const FOOD_SIZE: f32 = 6.0;
const DINO_SIZE: f32 = 6.0;

struct MainState {
    dino_x: Vec<f32>,
    dino_y: Vec<f32>,
    dino_dist: Vec<f32>,
    food_x: Vec<f32>,
    food_y: Vec<f32>,
    food_index_buffer: Vec<usize>,
    full: Vec<usize>,
    hungry: Vec<usize>,
}

impl MainState {
    fn new() -> GameResult<Self> {
        let mut rng = rand::thread_rng();
        let mut dino_x: Vec<f32> = Vec::with_capacity(NUM_DINOS);
        let mut dino_y: Vec<f32> = Vec::with_capacity(NUM_DINOS);
        let mut food_x: Vec<f32> = Vec::with_capacity(NUM_FOOD);
        let mut food_y: Vec<f32> = Vec::with_capacity(NUM_FOOD);
        let mut dino_dist: Vec<f32> = Vec::with_capacity(NUM_DINOS);
        let mut food_index_buffer: Vec<usize> = Vec::with_capacity(NUM_DINOS);

        let mut full: Vec<usize> = Vec::with_capacity(NUM_DINOS);
        let mut hungry: Vec<usize> = (0..NUM_DINOS).collect();

        // random dino pos
        for _ in 0..NUM_DINOS {
            dino_x.push(rng.gen_range(0.0..SCREEN_WIDTH));
            dino_y.push(rng.gen_range(0.0..SCREEN_HEIGHT));
        }

        // random food pos
        for _ in 0..NUM_FOOD {
            food_x.push(rng.gen_range(0.0..SCREEN_WIDTH));
            food_y.push(rng.gen_range(0.0..SCREEN_HEIGHT));
        }

        // dino dist
        for dino in 0..NUM_DINOS {
            let dino_x = dino_x[dino];
            let dino_y = dino_y[dino];
            let mut min_dist = f32::MAX;
            let mut mindex: usize = 0;
            for food in 0..NUM_FOOD {
                let dist = dist_of_xyxy(dino_x, dino_y, food_x[food], food_y[food]);
                if dist < min_dist {
                    min_dist = dist;
                    mindex = food;
                }
            }
            dino_dist.push(min_dist);
            food_index_buffer.push(mindex);
        }

        Ok(Self {
            dino_x,
            dino_y,
            dino_dist,
            food_x,
            food_y,
            food_index_buffer,
            full,
            hungry,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for dino in 0..NUM_DINOS {
            let dino_x = self.dino_x[dino];
            let dino_y = self.dino_y[dino];
            let dist = self.dino_dist[dino];
            let food_index = self.food_index_buffer[dino];
            let food_x = self.food_x[food_index];
            let food_y = self.food_y[food_index];

            // starting from 0 going counter clockwise
            // a = 0
            // b = pi/2 and so on

            let dists = [
                dist_of_xyxy(dino_x + dist, dino_y, food_x, food_y),
                dist_of_xyxy(dino_x, dino_y - dist, food_x, food_y),
                dist_of_xyxy(dino_x - dist, dino_y, food_x, food_y),
                dist_of_xyxy(dino_x, dino_y + dist, food_x, food_y),
            ];

            let mut min = f32::MAX;
            let mut second = f32::MAX;
            let mut min_quadrant: usize = 0;
            let
            for (index, dist) in dists.iter().enumerate() {
                if *dist < min {
                    second = min;
                    min = *dist;
                } else if *dist < second {
                    second = *dist;
                }
            }







        }
        // println!("fps {:?}", ctx.time.fps());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let dino_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            DINO_SIZE,
            2.0,
            Color::GREEN,
        )?;

        let food_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            FOOD_SIZE,
            2.0,
            Color::RED,
        )?;

        let radius_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(5.0),
            Vec2::new(0.0, 0.0),
            self.dino_dist[0],
            1.0,
            Color::BLACK,
        )?;

        canvas.draw(&radius_mesh, Vec2::new(self.dino_x[0], self.dino_y[0]));
        for index in 0..NUM_FOOD {
            canvas.draw(
                &food_mesh,
                Vec2::new(self.food_x[index], self.food_y[index]),
            );
        }
        for index in 0..NUM_DINOS {
            canvas.draw(
                &dino_mesh,
                Vec2::new(self.dino_x[index], self.dino_y[index]),
            );
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn dist_of_xyxy(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

fn main() -> GameResult {
    assert!(NUM_DINOS != 0);
    assert!(NUM_FOOD != 0);
    let cb = ggez::ContextBuilder::new("Dino Beast", "Mac")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Dino Beast")
                .vsync(false),
        )
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

const Q1: [f32; 91] = [0.0, 0.017453292, 0.034906585, 0.05235988, 0.06981317, 0.08726647, 0.10471976, 0.122173056, 0.13962634, 0.15707964, 0.17453294, 0.19198623, 0.20943952, 0.22689281, 0.24434611, 0.2617994, 0.27925268, 0.296706, 0.31415927, 0.33161256, 0.34906587, 0.36651915, 0.38397247, 0.40142572, 0.41887903, 0.43633232, 0.45378563, 0.4712389, 0.48869222, 0.5061455, 0.5235988, 0.5410521, 0.55850536, 0.57595867, 0.593412, 0.6108653, 0.62831855, 0.64577186, 0.6632251, 0.6806784, 0.69813174, 0.715585, 0.7330383, 0.75049156, 0.76794493, 0.7853982, 0.80285144, 0.8203048, 0.83775806, 0.8552114, 0.87266463, 0.890118, 0.90757126, 0.9250245, 0.9424778, 0.9599311, 0.97738445, 0.9948377, 1.012291, 1.0297443, 1.0471976, 1.0646509, 1.0821042, 1.0995575, 1.1170107, 1.134464, 1.1519173, 1.1693707, 1.186824, 1.2042772, 1.2217306, 1.2391838, 1.2566371, 1.2740904, 1.2915437, 1.308997, 1.3264502, 1.3439035, 1.3613569, 1.3788102, 1.3962635, 1.4137167, 1.43117, 1.4486233, 1.4660766, 1.4835298, 1.5009831, 1.5184366, 1.5358899, 1.553343, 1.5707964];
const Q2: [f32; 90] = [1.5882497, 1.6057029, 1.6231562, 1.6406096, 1.6580629, 1.6755161, 1.6929694, 1.7104228, 1.727876, 1.7453293, 1.7627826, 1.780236, 1.7976892, 1.8151425, 1.8325958, 1.850049, 1.8675023, 1.8849556, 1.9024088, 1.9198622, 1.9373156, 1.9547689, 1.9722221, 1.9896754, 2.0071287, 2.024582, 2.0420353, 2.0594885, 2.076942, 2.0943952, 2.1118484, 2.1293018, 2.146755, 2.1642084, 2.1816616, 2.199115, 2.2165682, 2.2340214, 2.2514749, 2.268928, 2.2863812, 2.3038347, 2.321288, 2.3387413, 2.3561945, 2.373648, 2.3911011, 2.4085543, 2.4260077, 2.4434612, 2.4609144, 2.4783676, 2.495821, 2.5132742, 2.5307274, 2.5481808, 2.5656343, 2.5830874, 2.6005406, 2.617994, 2.6354473, 2.6529005, 2.670354, 2.687807, 2.7052603, 2.7227137, 2.7401671, 2.7576203, 2.7750735, 2.792527, 2.8099802, 2.8274333, 2.8448865, 2.86234, 2.8797934, 2.8972466, 2.9147, 2.9321532, 2.9496067, 2.9670596, 2.984513, 3.0019662, 3.0194197, 3.036873, 3.0543263, 3.0717797, 3.0892327, 3.106686, 3.1241393, 3.1415927];
const Q3: [f32; 90] = [3.1590462, 3.1764994, 3.1939528, 3.2114058, 3.2288592, 3.2463124, 3.2637658, 3.2812192, 3.2986724, 3.3161259, 3.3335788, 3.3510323, 3.3684855, 3.385939, 3.4033923, 3.4208455, 3.438299, 3.455752, 3.4732053, 3.4906585, 3.508112, 3.5255651, 3.5430186, 3.560472, 3.577925, 3.5953784, 3.6128316, 3.630285, 3.6477382, 3.6651917, 3.682645, 3.700098, 3.7175512, 3.7350047, 3.752458, 3.7699113, 3.7873647, 3.8048177, 3.822271, 3.8397243, 3.8571777, 3.8746312, 3.8920844, 3.9095378, 3.9269907, 3.9444442, 3.9618974, 3.9793508, 3.9968042, 4.0142574, 4.0317106, 4.049164, 4.066617, 4.0840707, 4.101524, 4.118977, 4.1364307, 4.153884, 4.1713367, 4.1887903, 4.2062435, 4.2236967, 4.2411504, 4.2586036, 4.276057, 4.29351, 4.310963, 4.328417, 4.34587, 4.363323, 4.3807764, 4.39823, 4.415683, 4.4331365, 4.4505897, 4.468043, 4.4854965, 4.5029497, 4.520403, 4.537856, 4.5553093, 4.5727625, 4.590216, 4.6076694, 4.6251225, 4.642576, 4.660029, 4.6774826, 4.694936, 4.712389];
const Q4: [f32; 90] = [4.7298427, 4.747296, 4.764749, 4.7822022, 4.7996554, 4.8171086, 4.8345623, 4.8520155, 4.8694687, 4.8869224, 4.904375, 4.9218287, 4.939282, 4.956735, 4.974189, 4.991642, 5.009095, 5.0265484, 5.0440016, 5.061455, 5.0789084, 5.0963616, 5.113815, 5.1312685, 5.148721, 5.166175, 5.183628, 5.2010813, 5.2185345, 5.235988, 5.253441, 5.2708945, 5.2883477, 5.305801, 5.3232546, 5.340708, 5.358161, 5.375614, 5.3930674, 5.4105206, 5.427974, 5.4454274, 5.4628806, 5.4803343, 5.497787, 5.5152407, 5.532694, 5.550147, 5.5676007, 5.585054, 5.602507, 5.6199603, 5.6374135, 5.6548667, 5.6723204, 5.689773, 5.7072268, 5.72468, 5.742133, 5.759587, 5.77704, 5.794493, 5.8119464, 5.8294, 5.8468533, 5.8643064, 5.88176, 5.8992133, 5.9166665, 5.934119, 5.951573, 5.969026, 5.9864793, 6.0039325, 6.021386, 6.0388393, 6.0562925, 6.073746, 6.0911994, 6.1086526, 6.1261063, 6.1435595, 6.161012, 6.1784654, 6.1959186, 6.213372, 6.2308254, 6.2482786, 6.2657323, 6.2831855];
