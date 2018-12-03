use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use na;
use na::{Isometry2, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::query::{Ray, RayCast};
use ncollide2d::shape::{Ball, Cuboid};
use piston_window::*;
use std::f64;

type Point = Point2<f64>;
type Isometry = Isometry2<f64>;

/// 車の動きを再現するsimulator
pub struct Simulator {
    display_size: (u32, u32),
    arena: Arena,
    eater: Eater,
}

impl Simulator {
    /// Simulatorインスタンスを生成する
    ///
    /// # Arguments
    /// * `display_size` - ウィンドウのサイズ
    pub fn new(display_size: (u32, u32)) -> Simulator {
        let arena = Arena::new(display_size.0 as f64, display_size.1 as f64);
        let orig_x = arena.visual_orig.0 + 50.0;
        let orig_y = (arena.visual_orig.1 + arena.height) / 2.0 - 30.0;
        let eater = Eater::new(orig_x, orig_y, 15.0, display_size.1 as f64);

        Simulator {
            display_size: display_size,
            arena: arena,
            eater: eater,
        }
    }
    pub fn run<F>(&mut self, mut update: F)
    where
        F: FnMut(&mut Eater, &Arena),
    {
        let mut window: PistonWindow = WindowSettings::new("subsumption model", self.display_size)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
        while let Some(e) = window.next() {
            self.eater.render(&mut window, &e, &mut self.arena, &mut update);
        }
    }
}

/// 車が動き回れるエリア  
/// piston座標系は原点を左上、右方向にx、下方向にy、半時計回りにθとする  
/// ncollide座標系は原点を左下、右方向にx、上方向にy、半時計回りにθとする
pub struct Arena {
    visual_orig: (f64, f64),  // 描画上の原点
    width: f64,               // arenaの横幅
    height: f64,              //arenaの縦幅
    window_height: f64,       // windowの縦幅
    cuboid: Cuboid<f64>, // 長方形型のobjectでarenaと同じ大きさ。衝突や車との距離を検知する
    transformed: Isometry, // cuboidと一緒に使って衝突や車との距離を検知する
    obstacles: Vec<Obstacle>, // arena内の障害物
    feeds: Vec<Feed>,    // arena内のゴミ
}
impl Arena {
    /// arenaを生成する
    /// # Arguments
    /// * `window_x` - ウィンドウの横幅(arenaの横幅ではない)
    /// * `window_y` - ウィンドウの縦幅(arenaの縦幅ではない)
    pub fn new(window_x: f64, window_y: f64) -> Arena {
        let resize = 0.9; // arenaの大きさをウィンドウの何%にするか
        let x_diff = window_x * (1.0 - resize) * 0.5;
        let y_diff = window_y * (1.0 - resize) * 0.5;
        let transformed = Isometry::new(
            Vector2::new(
                x_diff + (window_x - x_diff * 2.0) / 2.0,
                y_diff + (window_y - y_diff * 2.0) / 2.0,
            ),
            na::zero(),
        );
        let obstacles = vec![
            Obstacle::new(150.0, 225.0, 30.0, window_y),
            Obstacle::new(300.0, 100.0, 30.0, window_y),
            Obstacle::new(450.0, 225.0, 30.0, window_y),
            Obstacle::new(375.0, 380.0, 30.0, window_y),
            Obstacle::new(225.0, 380.0, 30.0, window_y),
        ];
        let feeds = vec![
            Feed::new(100.0, 100.0, 3.0, window_y),
            Feed::new(110.0, 400.0, 3.0, window_y),
            Feed::new(120.0, 150.0, 3.0, window_y),
            Feed::new(130.0, 300.0, 3.0, window_y),
            Feed::new(140.0, 170.0, 3.0, window_y),
            Feed::new(150.0, 270.0, 3.0, window_y),
            Feed::new(160.0, 275.0, 3.0, window_y),
            Feed::new(170.0, 300.0, 3.0, window_y),
            Feed::new(180.0, 305.0, 3.0, window_y),
            Feed::new(190.0, 192.0, 3.0, window_y),
            Feed::new(200.0, 100.0, 3.0, window_y),
            Feed::new(200.0, 200.0, 3.0, window_y),
            Feed::new(210.0, 200.0, 3.0, window_y),
            Feed::new(225.0, 109.0, 3.0, window_y),
            Feed::new(235.0, 120.0, 3.0, window_y),
            Feed::new(250.0, 450.0, 3.0, window_y),
            Feed::new(275.0, 312.0, 3.0, window_y),
            Feed::new(290.0, 290.0, 3.0, window_y),
            Feed::new(300.0, 140.0, 3.0, window_y),
            Feed::new(315.0, 150.0, 3.0, window_y),
            Feed::new(325.0, 288.0, 3.0, window_y),
            Feed::new(335.0, 192.0, 3.0, window_y),
            Feed::new(345.0, 105.0, 3.0, window_y),
            Feed::new(350.0, 222.0, 3.0, window_y),
            Feed::new(365.0, 333.0, 3.0, window_y),
            Feed::new(385.0, 111.0, 3.0, window_y),
            Feed::new(395.0, 59.0, 3.0, window_y),
            Feed::new(400.0, 444.0, 3.0, window_y),
            Feed::new(405.0, 256.0, 3.0, window_y),
            Feed::new(415.0, 321.0, 3.0, window_y),
            Feed::new(425.0, 123.0, 3.0, window_y),
            Feed::new(435.0, 190.0, 3.0, window_y),
            Feed::new(450.0, 400.0, 3.0, window_y),
            Feed::new(470.0, 80.0, 3.0, window_y),
        ];

        Arena {
            visual_orig: (x_diff, y_diff),
            width: window_x - x_diff * 2.0,
            height: window_y - y_diff * 2.0,
            cuboid: Cuboid::new(Vector2::new(
                (window_x - x_diff * 2.0) * 0.5,
                (window_y - y_diff * 2.0) * 0.5,
            )),
            window_height: window_y,
            transformed: transformed,
            obstacles: obstacles,
            feeds: feeds,
        }
    }

    /// arena内のある座標から最も近い壁との距離を取得する
    fn distance_to_nearest_wall(&self, point: &Point) -> f64 {
        -self.cuboid.distance_to_point(&self.transformed, point, false)
    }

    /// arena内のある座標からある方角に直線上の光線を発射したときに壁にぶつかるまでの時間を取得する  
    /// ある位置から任意の方角の壁との距離を図るのに使う
    fn toi_in_direction(&self, ray: &Ray<f64>) -> f64 {
        let intersection = self.cuboid.toi_and_normal_with_ray(&self.transformed, ray, false);
        intersection.map_or(f64::MAX, |intersection| intersection.toi)
    }

    fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>) {
        Rectangle::new_border(Color::LightCyan.to_rgb(), 1.5).draw(
            [self.visual_orig.0, self.visual_orig.1, self.width, self.height], // (x, y, width, height)
            &c.draw_state,
            c.transform,
            g,
        );
        for o in &self.obstacles {
            o.draw(c, g);
        }
        for f in &self.feeds {
            f.draw(c, g);
        }
    }
}

/// 車
pub struct Eater {
    radius: f64, // 車の半径
    x: f64,      // piston座標系におけるx座標
    y: f64,      // piston座標系におけるy座標
    /// 車体左側のセンサー
    pub left_sensor: Sensor,
    /// 車体右側のセンサー
    pub right_sensor: Sensor,
    /// 左車輪の速度
    pub left_speed: f64,
    /// 右車輪の速度
    pub right_speed: f64,
    angle: f64,
    back: i32,
    eating: bool,
    color: Color,
}

impl Object for Eater {
    fn new(x: f64, y: f64, radius: f64, window_height: f64) -> Eater {
        let y = window_height - y;
        let field_of_vision = 120.0_f64.to_radians();
        Eater {
            radius: radius,
            x: x,
            y: y,
            left_sensor: Sensor::new((x, y), field_of_vision / 2.0, Color::Gray, radius * 4.0, radius),
            right_sensor: Sensor::new((x, y), -field_of_vision / 2.0, Color::Gray, radius * 4.0, radius),
            left_speed: 1.0,
            right_speed: 1.0,
            angle: 0.0,
            back: 0,
            eating: false,
            color: Color::LightGreen,
        }
    }
}

impl Eater {
    fn render<E, F>(&mut self, w: &mut PistonWindow, e: &E, arena: &mut Arena, mut update_closure: F)
    where
        E: generic_event::GenericEvent,
        F: FnMut(&mut Eater, &Arena),
    {
        w.draw_2d(e, |c, g| {
            clear(Color::White.to_rgb(), g);

            let action = (self.left_speed, self.right_speed);
            let t = 1.0;

            self.eat(arena);

            // 後退中の場合
            if (self.back > 0 && !self.is_touching(arena)) || (self.back == 25 && self.is_touching(arena)) {
                let action = (-self.left_speed, -self.right_speed);
                let v = (action.0 + action.1) / 2.0;

                let delta_angle = delta_angle(action.0 * t, action.1 * t, self.radius);
                let trans_x = -v * t * (self.angle + delta_angle / 2.0).to_radians().cos();
                let trans_y = -v * t * (self.angle + delta_angle / 2.0).to_radians().sin();

                let next_angle = (self.angle + delta_angle) % 360.0;

                self.draw(c, g, &arena, next_angle);

                self.left_speed = 1.0;
                self.right_speed = 1.0;
                self.update(-trans_x, -trans_y, next_angle);
                self.back = self.back - 1;
                return;
            }

            // 障害物に接触したときは一定時間後退する。後退中に障害物に接触したときは後退をやめる
            if self.is_touching(&arena) && self.back == 0 {
                self.back = 25;
                return;
            }

            // 対向2輪ロボットの移動方法
            // http://www.mech.tohoku-gakuin.ac.jp/rde/contents/course/robotics/wheelrobot.html

            let velocity_l = action.0 * t; // 左車輪の移動速度
            let velocity_r = action.1 * t; // 右車輪の移動速度
            let omega = delta_angle(velocity_l, velocity_r, self.radius); // 車体の角度の変化量。左右の車輪速度の違いと、車体の半径から求める

            // x座標とy座標にどれだけ動くかの差分を求めている
            let (trans_x, trans_y) = if action.1 != action.0 {
                // 左右の車輪の速度が違うとき
                let velocity = (velocity_l + velocity_r) / 2.0; // delta-l 車中心部分の移動速度
                let roh = velocity * (360.0 / omega) * (1.0 / (2.0 * 3.14)); // 車体が描く円弧の回転半径(ρ)
                let delta_l_dash = 2.0 * roh * (omega / 2.0).to_radians().sin(); //1フレームにおける車体の中心の移動距離(ΔL')
                (
                    delta_l_dash * (self.angle + omega / 2.0).to_radians().cos(), // 1フレームにおける車体のx座標の移動距離
                    delta_l_dash * (self.angle + omega / 2.0).to_radians().sin(), // 1フレームにおける車体のy座標の移動距離
                )
            } else {
                // 左車輪と右車輪が同速度の場合は直進するだけなので、現在の向きを維持したまま直進している
                let v = action.0 * t; // 移動速度(action.1を使っても同じ)
                (
                    v * self.angle.to_radians().cos(), // 移動速度にcosθ(θは今向いている角度)をかければx方向にどれだけ動いたかが分かる
                    v * self.angle.to_radians().sin(), // 移動速度にsinθ(θは今向いている角度)をかければy方向にどれだけ動いたかが分かる
                )
            };

            let next_angle = (self.angle + omega) % 360.0; // 次のフレームでの車体の向き
            self.draw(c, g, &arena, next_angle);

            update_closure(self, &arena);
            self.update(trans_x, trans_y, next_angle);
        });
    }

    fn eat(&mut self, arena: &mut Arena) {
        let mut eating = false;
        let point = Point::new(self.x, arena.window_height - self.y);
        for feed in &mut arena.feeds {
            if feed.distance_from_point(&point) <= self.radius {
                feed.life -= 1;
                eating = true;
            }
        }
        self.eating = eating;
        arena.feeds.retain(|feed| feed.life != 0);
    }

    /// 左右のセンサーからの情報を取得する
    ///
    /// # Arguments
    /// * `arena` - フロアの情報
    /// # Return
    /// * ((左センサーと障害物との距離, 右センサーと障害物との距離), 車体がゴミを食べているかどうか)
    pub fn sensor_data(&self, arena: &Arena) -> ((f64, f64), bool) {
        (
            (
                self.left_sensor.data(arena).unwrap_or(0.0),
                self.right_sensor.data(arena).unwrap_or(0.0),
            ),
            self.is_eating(),
        )
    }

    fn is_eating(&self) -> bool {
        self.eating
    }

    fn is_touching(&self, arena: &Arena) -> bool {
        let point = Point::new(self.x, self.y);
        let mut nearest_distance = arena.distance_to_nearest_wall(&point);

        let point = Point::new(self.x, arena.window_height - self.y);
        for obstacle in &arena.obstacles {
            let distance = obstacle.distance_from_point(&point);
            if nearest_distance > distance {
                nearest_distance = distance
            }
        }
        nearest_distance < (self.radius)
    }

    pub fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>, arena: &Arena, next_angle: f64) {
        arena.draw(c, g);
        self.left_sensor.draw(c, g, next_angle);
        self.right_sensor.draw(c, g, next_angle);
        let square = ellipse::circle(self.x, self.y, self.radius);
        ellipse(self.color.to_rgb(), square, c.transform, g);

        let center_line_color = Color::Gray.to_rgb();
        let zero_center = [0.0, 0.0, self.radius, 0.0];
        // 初期値じゃなくてtransで移さないとrotateの原点が移らない?
        let transed = c.transform.trans(self.x, self.y);
        line(center_line_color, 1.0, zero_center, transed.rot_deg(-next_angle), g);
    }

    pub fn update_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn update(&mut self, trans_x: f64, trans_y: f64, next_angle: f64) {
        self.x = self.x + trans_x;
        self.y = self.y - trans_y; // trans_yはncollide座標系の値なので、piston座標系に合わせて反転させる
        self.angle = next_angle;
        self.left_sensor.update((self.x, self.y), self.angle);
        self.right_sensor.update((self.x, self.y), self.angle);
    }
}

pub struct Sensor {
    x: f64,
    y: f64,
    field_of_vision: f64,
    angle: f64,
    color: Color,
    length: f64,
    radius: f64,
}

impl Sensor {
    // 壁及び障害物の内、最も近いものとの距離を正規化して取得する
    // 近いほど1に近くなり、遠いと0になる
    pub fn data(&self, arena: &Arena) -> Option<f64> {
        if let Some(distance) = self.nearest_distance(arena) {
            if distance >= self.length {
                return Some(0.0);
            }
            return Some(1.0 - (distance - self.radius) / (self.length - self.radius));
        }
        None
    }

    fn new(orig: (f64, f64), field_of_vision: f64, color: Color, length: f64, radius: f64) -> Sensor {
        Sensor {
            x: orig.0,
            y: orig.1,
            field_of_vision: field_of_vision,
            angle: 0.0,
            color,
            length,
            radius,
        }
    }

    fn update(&mut self, pos: (f64, f64), angle: f64) {
        self.x = pos.0;
        self.y = pos.1;
        self.angle = angle;
    }

    fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>, next_angle: f64) {
        let left_sensor = [
            0.0,
            0.0,
            self.length * (self.field_of_vision).cos(),
            -self.length * (self.field_of_vision).sin(),
        ];
        let transed = c.transform.trans(self.x, self.y);
        line(self.color.to_rgb(), 1.0, left_sensor, transed.rot_deg(-next_angle), g);
    }

    // 壁及び障害物の内、最も近いものとの距離
    fn nearest_distance(&self, arena: &Arena) -> Option<f64> {
        let theta = self.field_of_vision + self.angle.to_radians();
        let dir_x = theta.cos();
        let dir_y = theta.sin();
        let y = arena.window_height - self.y;
        let ray = Ray::new(Point::new(self.x, y), Vector2::new(dir_x, dir_y));
        let mut closest_toi: f64 = arena.toi_in_direction(&ray); // センサーと壁との距離を衝突時間で表す

        // obstacles distance
        for obstacle in &arena.obstacles {
            let ball = Ball::new(obstacle.radius);
            let transformed = Isometry::new(Vector2::new(obstacle.x, arena.window_height - obstacle.y), na::zero());
            let intersection = ball.toi_and_normal_with_ray(&transformed, &ray, false);
            if let Some(i) = intersection {
                if closest_toi > i.toi {
                    closest_toi = i.toi;
                }
            }
        }

        Some(self.intersection_distance(dir_x, dir_y, closest_toi, y))
    }

    fn intersection_distance(&self, dir_x: f64, dir_y: f64, toi: f64, y: f64) -> f64 {
        let i_point = (self.x + dir_x * toi, y + dir_y * toi);
        ((self.x - i_point.0).powi(2) + (y - i_point.1).powi(2)).sqrt()
    }
}

struct Obstacle {
    x: f64,
    y: f64,
    radius: f64,
    ball: Ball<f64>,
    transformed: Isometry,
}

impl Object for Obstacle {
    fn new(x: f64, y: f64, radius: f64, window_height: f64) -> Obstacle {
        Obstacle {
            x,
            y,
            radius,
            ball: Ball::new(radius),
            transformed: Isometry::new(Vector2::new(x, window_height - y), na::zero()),
        }
    }
}

impl Target for Obstacle {
    fn distance_from_point(&self, point: &Point) -> f64 {
        self.ball.distance_to_point(&self.transformed, &point, false)
    }

    fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>) {
        let square = ellipse::circle(self.x, self.y, self.radius);
        ellipse(Color::LightCyan.to_rgb(), square, c.transform, g);
    }
}

struct Feed {
    x: f64,
    y: f64,
    radius: f64,
    life: u32,
    ball: Ball<f64>,
    transformed: Isometry,
}

impl Object for Feed {
    fn new(x: f64, y: f64, radius: f64, window_height: f64) -> Feed {
        Feed {
            x,
            y,
            radius,
            life: 50,
            ball: Ball::new(radius),
            transformed: Isometry::new(Vector2::new(x, window_height - y), na::zero()),
        }
    }
}

impl Target for Feed {
    fn distance_from_point(&self, point: &Point) -> f64 {
        self.ball.distance_to_point(&self.transformed, &point, false)
    }

    fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>) {
        let square = ellipse::circle(self.x, self.y, self.radius);
        ellipse(Color::Black.to_rgb(), square, c.transform, g);
    }
}

fn delta_angle(delta_left: f64, delta_right: f64, radius: f64) -> f64 {
    ((delta_right - delta_left) / (radius * 2.0)) * (180.0 / 3.14) // 1フレーム当たりの車体の角度の差分を求める式。結果はradなので度数に直している。
}

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    LightCyan,
    LightGreen,
    Gray,
    White,
    Black,
}

impl Color {
    pub fn to_rgb(&self) -> [f32; 4] {
        match *self {
            Color::Red => [1.0, 0.0, 0.0, 1.0],
            Color::Green => [0.0, 1.0, 0.0, 1.0],
            Color::Blue => [0.0, 0.0, 1.0, 1.0],
            Color::LightCyan => [230.0 / 250.0, 230.0 / 250.0, 250.0 / 250.0, 1.0],
            Color::LightGreen => [144.0 / 255.0, 238.0 / 255.0, 144.0 / 255.0, 1.0],
            Color::Gray => [0.5, 0.5, 0.5, 1.0],
            Color::White => [1.0, 1.0, 1.0, 1.0],
            Color::Black => [0.0, 0.0, 0.0, 1.0],
        }
    }
}

trait Object {
    fn new(x: f64, y: f64, radius: f64, window_height: f64) -> Self;
}

trait Target: Object {
    fn distance_from_point(&self, point: &Point) -> f64;
    fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>);
}
