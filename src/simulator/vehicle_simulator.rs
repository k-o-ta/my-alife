use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use na;
use na::{Isometry2, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::query::{Ray, RayCast, RayInterferencesCollector};
use ncollide2d::shape::{Ball, Cuboid};
use piston_window::*;
use std::{thread, time};
// gfx_graphics::back_end

pub struct Simulator {
    display_size: (u32, u32),
    arena: Arena,
    eater: Eater,
}
impl Simulator {
    pub fn new(display_size: (u32, u32)) -> Simulator {
        let arena = Arena::new(display_size.0 as f64, display_size.1 as f64);
        let eater = Eater::new((100.0, 100.0), display_size.1 as f64);

        Simulator {
            display_size: display_size,
            arena: arena,
            eater: eater,
        }
    }
    pub fn run(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("Hello Piston!", self.display_size)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
        while let Some(e) = window.next() {
            self.eater.render(
                &mut window,
                &e,
                (0.0 as f64, 0.0 as f64), // unused
                &self.arena,
            );
        }
    }
}

pub struct Arena {
    nw: (f64, f64),
    width: f64,
    height: f64,
    window_height: f64,
    pub cuboid: Cuboid<f64>,
    pub transformed: Isometry2<f64>,
    obstacles: Vec<Obstacle>,
}
impl Arena {
    pub fn new(window_x: f64, window_y: f64) -> Arena {
        let resize = 0.9;
        let x_diff = window_x * (1.0 - resize) * 0.5;
        let y_diff = window_y * (1.0 - resize) * 0.5;
        // let transformed = Isometry2::new(Vector2::new(x_diff, y_diff), na::zero());
        let transformed = Isometry2::new(
            Vector2::new(
                x_diff + (window_x - x_diff * 2.0) / 2.0,
                y_diff + (window_y - y_diff * 2.0) / 2.0,
            ),
            na::zero(),
        );
        // let transformed = Isometry2::identity();
        // let _t = ;
        println!(
            "x_diff: {}, y_diff: {}, cubic_pos: {:?},se: {:?}",
            x_diff,
            y_diff,
            (window_x - x_diff * 2.0, window_y - y_diff * 2.0),
            (
                (window_x - x_diff * 2.0) * 0.5 + x_diff + (window_x - x_diff * 2.0) / 2.0,
                (window_y - y_diff * 2.0) * 0.5 + y_diff + (window_y - y_diff * 2.0) / 2.0,
            )
        );
        let obstacles = vec![Obstacle {
            x: 200.0,
            y: 220.0,
            radius: 50.0,
        }];

        Arena {
            nw: (x_diff, y_diff),
            width: window_x - x_diff * 2.0,
            height: window_y - y_diff * 2.0,
            cuboid: Cuboid::new(Vector2::new(
                // (window_x - x_diff * 2.0) * 0.5,
                // (window_y - y_diff * 2.0) * 0.5,
                (window_x - x_diff * 2.0) * 0.5,
                (window_y - y_diff * 2.0) * 0.5,
            )),
            window_height: window_y,
            transformed: transformed,
            obstacles: obstacles,
        }
    }

    pub fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>) {
        let blue = [0.0, 0.0, 1.0, 1.0];
        Rectangle::new_border(blue, 1.5).draw(
            [self.nw.0, self.nw.1, self.width, self.height], // (x, y, width, height)
            &c.draw_state,
            c.transform,
            g,
        );
        for o in &self.obstacles {
            o.draw(c, g);
        }
    }
}
pub struct Eater {
    radius: f64,
    x: f64,
    y: f64,
    field_of_vision: f64,
    left_sensor: Sensor,
    right_sensor: Sensor,
    left_speed: f64,
    right_speed: f64,
    angle: f64,
    next_angle: f64,
    back: i32,
}

use std::f64;
impl Eater {
    pub fn new(orig: (f64, f64), height: f64) -> Eater {
        let radius = 50.0;
        let x = orig.0;
        let y = height - orig.1;
        let field_of_vision = 120.0_f64.to_radians();
        Eater {
            radius: radius,
            field_of_vision: field_of_vision,
            x: x,
            y: y,
            left_speed: 2.0,
            left_sensor: Sensor::new(
                (x, y),
                field_of_vision / 2.0,
                (x + (field_of_vision / 2.0).cos(), y + (field_of_vision / 2.0).sin()),
                [0.5, 0.5, 0.5, 1.0],
                radius * 2.5,
                radius,
            ),
            right_sensor: Sensor::new(
                (x, y),
                -field_of_vision / 2.0,
                (x + (-field_of_vision / 2.0).cos(), y + (-field_of_vision / 2.0).sin()),
                [0.5, 0.5, 0.5, 1.0],
                radius * 2.5,
                radius,
            ),
            right_speed: 1.0,
            angle: 0.0,
            next_angle: 0.0,
            back: 0,
        }
    }
    pub fn render<E>(&mut self, w: &mut PistonWindow, e: &E, action: (f64, f64), arena: &Arena)
    where
        E: generic_event::GenericEvent,
    {
        w.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            let action = (self.left_speed , self.right_speed );
            let t = 1.0;

            if self.back > 0 {
              let action = (-self.left_speed , -self.right_speed );
              let v = (action.0 + action.1) / 2.0;

              let delta_angle = delta_angle(action.0 * t, action.1 * t, self.radius);
              let trans_x = -v * t *(self.angle + delta_angle / 2.0).to_radians().cos();
              let trans_y = -v * t *(self.angle + delta_angle / 2.0).to_radians().sin();

              let next_angle = (self.angle + delta_angle) % 360.0;

              self.draw(c, g, &arena, next_angle);

              self.left_speed = 1.0;
              self.right_speed = 1.0;
              self.update(-trans_x, -trans_y, next_angle);
              self.back = self.back - 1;
              // println!("back!!!!!!!!!!!!: {}, trans_x: {}, trans_y: {}, self_x: {}, self_y: {}, angle: {}, delta_angle: {}", self.back, trans_x, trans_y,  self.x, self.y, self.angle, delta_angle);
              return ;
            }

            if self.is_touched(&arena)  {
              self.back = 25;
              return;
            }

            let v = (action.0 + action.1) / 2.0;

            let delta_l = action.0 * t;
            let delta_r = action.1 * t;
            let delta_angle = delta_angle(delta_l, delta_r, self.radius); // dleta-theta

            let next_angle = (self.angle + delta_angle) % 360.0;
            let (trans_x, trans_y) = if action.1 != action.0 {
              let l = (delta_l + delta_r) / 2.0; // delta-l
              let p =  l * (360.0/ delta_angle) * (1.0/(2.0*3.14));
              let delta_l_dash = 2.0 * p * (delta_angle / 2.0).to_radians().sin();
              // println!("delta_l: {}, delta_r: {}, delata_angle: {}, p: {}, delta_l :{}", action.0, action.1,delta_angle,  p, delta_l_dash);
              (delta_l_dash * (self.angle + delta_angle / 2.0).to_radians().cos(), delta_l_dash * (self.angle + delta_angle / 2.0).to_radians().sin())
            } else {
              // println!("delta_l: {}, delta_r: {}, angle: {}, delata_angle: {}", action.0, action.1,self.angle, delta_angle);
              (v * t *(self.angle + delta_angle / 2.0).to_radians().cos(), v * t *(self.angle + delta_angle / 2.0).to_radians().sin())
            };

            self.draw(c, g, &arena, next_angle);

            if let Some(data) = self.left_sensor.data(arena) {
              self.left_speed = 2.0 + 2.0 * data;
            }
            if let Some(data) = self.right_sensor.data(arena) {
              self.right_speed = 2.0 + 2.0 * data;
            }
            self.update(trans_x, trans_y, next_angle);
        });
    }
    pub fn is_touched(&self, arena: &Arena) -> bool {
        let point = Point2::new(self.x, self.y);
        let mut distance = -arena.cuboid.distance_to_point(&arena.transformed, &point, false);

        for obstacle in &arena.obstacles {
            // let point = Point2::new(self.x, arena.window_height - self.y);
            let point = Point2::new(self.x, arena.window_height - self.y);
            let ball = Ball::new(obstacle.radius);
            let transformed = Isometry2::new(Vector2::new(obstacle.x, arena.window_height - obstacle.y), na::zero());
            // let transformed = Isometry2::new(Vector2::new(obstacle.x, obstacle.y), na::zero());
            let o_distance = ball.distance_to_point(&transformed, &point, false);
            // println!("o distance: {}, arana_distance: {}", o_distance, distance);
            if distance > o_distance {
                println!("o distance: {}", o_distance);
                distance = o_distance
                // thread::sleep(time::Duration::from_millis(3000));
            }
        }
        distance < (self.radius)
    }

    pub fn is_collide(&self, arena: &Arena) -> bool {
        let l = self.left_sensor.is_collide(arena);
        let r = self.right_sensor.is_collide(arena);
        l || r
    }

    pub fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>, arena: &Arena, next_angle: f64) {
        arena.draw(c, g);
        self.left_sensor.draw(c, g, next_angle);
        self.right_sensor.draw(c, g, next_angle);
        let square = ellipse::circle(self.x, self.y, self.radius);
        let mut color = if self.is_collide(arena) {
            [1.0, 1.0, 0.0, 1.0] // yellow
        } else {
            [0.0, 1.0, 0.0, 1.0] // green
        };
        if self.is_touched(arena) {
            color = [1.0, 0.0, 0.0, 1.0] //red
        };
        ellipse(color, square, c.transform, g);

        let center_line_color = [0.5, 0.5, 0.5, 1.0];
        let zero_center = [0.0, 0.0, self.radius, 0.0];
        // 初期値じゃなくてtransで移さないとrotateの原点が移らない?
        let transed = c.transform.trans(self.x, self.y);
        line(center_line_color, 1.0, zero_center, transed.rot_deg(-next_angle), g);
    }
    pub fn update(&mut self, trans_x: f64, trans_y: f64, next_angle: f64) {
        self.x = self.x + trans_x;
        self.y = self.y - trans_y;
        self.angle = next_angle;
        self.left_sensor.update((self.x, self.y), self.angle);
        self.right_sensor.update((self.x, self.y), self.angle);
    }
}
struct Sensor {
    x: f64,
    y: f64,
    field_of_vision: f64,
    dir: (f64, f64),
    angle: f64,
    ray: Ray<f64>,
    color: [f32; 4],
    length: f64,
    radius: f64,
}
impl Sensor {
    pub fn new(
        orig: (f64, f64),
        field_of_vision: f64,
        dir: (f64, f64),
        color: [f32; 4],
        length: f64,
        radius: f64,
    ) -> Sensor {
        let ray = Ray::new(Point2::new(orig.0, orig.1), Vector2::new(dir.0, dir.1));
        Sensor {
            x: orig.0,
            y: orig.1,
            field_of_vision: field_of_vision,
            dir: dir,
            angle: 0.0,
            ray,
            color,
            length,
            radius,
        }
    }
    pub fn update(&mut self, pos: (f64, f64), angle: f64) {
        self.x = pos.0;
        self.y = pos.1;
        self.angle = angle;
    }
    pub fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>, next_angle: f64) {
        let left_sensor = [
            0.0,
            0.0,
            self.length * (self.field_of_vision).cos(),
            -self.length * (self.field_of_vision).sin(),
        ];
        let transed = c.transform.trans(self.x, self.y);
        line(self.color, 1.0, left_sensor, transed.rot_deg(-next_angle), g);
    }
    pub fn is_collide(&self, arena: &Arena) -> bool {
        if let Some(distance) = self.distance(arena) {
            let collide = distance < self.length;
            // println!("distance: {}, length: {}", distance, self.length);
            if collide {
                // thread::sleep(time::Duration::from_millis(3000));
            }
            return collide;
        }
        false
    }

    pub fn data(&self, arena: &Arena) -> Option<f64> {
        if let Some(distance) = self.distance(arena) {
            if distance >= self.length {
                return Some(0.0);
            }
            return Some(1.0 - (distance - self.radius) / (self.length - self.radius));
        }
        None
    }

    fn distance(&self, arena: &Arena) -> Option<f64> {
        let theta = self.field_of_vision + self.angle.to_radians();
        let dir_x = theta.cos();
        let dir_y = theta.sin();
        let y = arena.window_height - self.y;
        let ray = Ray::new(Point2::new(self.x, y), Vector2::new(dir_x, dir_y));
        let inter = arena.cuboid.toi_and_normal_with_ray(&arena.transformed, &ray, false);
        let mut closest_toi: f64 = inter.map_or(f64::MAX, |intersection| intersection.toi);
        // obstacles distance
        for obstacle in &arena.obstacles {
            let ball = Ball::new(obstacle.radius);
            let transformed = Isometry2::new(Vector2::new(obstacle.x, arena.window_height - obstacle.y), na::zero());
            // let transformed = Isometry2::new(Vector2::new(obstacle.x, obstacle.y), na::zero());
            let intersection = ball.toi_and_normal_with_ray(&transformed, &ray, false);
            if let Some(i) = intersection {
                if closest_toi > i.toi {
                    closest_toi = i.toi;
                }
            }
        }
        // println!(
        //     "time: {}, distance: {}",
        //     closest_toi,
        //     self.intersection_distance(dir_x, dir_y, closest_toi, y)
        // );
        Some(self.intersection_distance(dir_x, dir_y, closest_toi, y))
        // let closest_toi = arena
        //     .obstacles
        //     .iter()
        //     .map(|obstacle| {
        //         let ball = Ball::new(obstacle.radius);
        //         let transformed = Isometry2::new(Vector2::new(obstacle.x, obstacle.y), na::zero());
        //         ball.toi_and_normal_with_ray(&transformed, &ray, false)
        //             .map_or(f64::MAX, |intersection| intersection.toi)
        //     }).fold(0.0 / 0.0, |m, v| v.min(m));
        //
        // if closest_toi == f64::NAN && arena_toi == f64::NAN {
        //     None
        // } else if closest_toi == f64::NAN && arena_toi != f64::NAN {
        //     Some(self.intersection_distance(dir_x, dir_y, arena_toi, y))
        // } else {
        //     Some(self.intersection_distance(dir_x, dir_y, closest_toi, y))
        // }
        // if let Some(i) = inter {
        //     let i_point = (self.x + dir_x * i.toi, y + dir_y * i.toi);
        //     let distance = ((self.x - i_point.0).powi(2) + (y - i_point.1).powi(2)).sqrt();
        //     return Some(distance);
        // }
        // None
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
}

impl Obstacle {
    pub fn new(x: f64, y: f64, radius: f64) -> Obstacle {
        Obstacle { x, y, radius }
    }
    pub fn draw(&self, c: Context, g: &mut GfxGraphics<'_, Resources, CommandBuffer>) {
        let square = ellipse::circle(self.x, self.y, self.radius);
        let blue = [0.0, 0.0, 1.0, 1.0];
        ellipse(blue, square, c.transform, g);
    }
}

struct Feed {}

fn delta_angle(delta_left: f64, delta_right: f64, radius: f64) -> f64 {
    90.0 * (delta_right - delta_left) / (radius * 3.14)
}
