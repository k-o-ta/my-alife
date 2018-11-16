use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use na;
use na::{Isometry2, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::query::{Ray, RayCast, RayInterferencesCollector};
use ncollide2d::shape::Cuboid;
use piston_window::*;
use std::{thread, time};
// gfx_graphics::back_end
pub struct Arena {
    nw: (f64, f64),
    width: f64,
    height: f64,
    window_height: f64,
    pub cuboid: Cuboid<f64>,
    pub transformed: Isometry2<f64>,
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
            // let square2 = ellipse::circle(0.0, 0.0, 50.0);
            // let red = [1.0, 0.0, 0.0, 1.0];
            // let center2 = c.transform.trans((1140) as f64, (0) as f64);
            // // let center2 = c.transform;
            // ellipse(
            //     red.clone(),
            //     square2,
            //     center2,
            //     g,
            // );
            //
            let action = (self.left_speed , self.right_speed );
            arena.draw(c, g);
            let t = 1.0;
            if self.back > 0 {
            let action = (-self.left_speed , -self.right_speed );
            let v = (action.0 + action.1) / 2.0;
            let l_l = action.0 * t;
            let l_r = action.1 * t;
            // let delta_angle = (l_r - l_l) / (2.0 * self.radius); // dleta-theta
            let delta_angle = 90.0*(l_r -l_l) / (self.radius*3.14); // dleta-theta
            let next_x = v * self.angle.to_radians().cos();
            let next_y = v * self.angle.to_radians().sin();
              let trans_x = next_x * t *(self.angle + delta_angle / 2.0).to_radians().cos();
              let trans_y = next_y * t*(self.angle + delta_angle / 2.0).to_radians().sin();
              let mut next_angle = 0.0;
            if self.angle + delta_angle > 360.0 {
                 next_angle = self.angle + delta_angle - 360.0;
            } else {
                 next_angle = self.angle + delta_angle;
            }


            let transed = c.transform.trans(self.x, self.y);
            // sensor
            let left_sensor = [
                0.0,
                0.0,
                self.left_sensor.length * (self.field_of_vision / 2.0).cos(),
                // self.left_sensor.length * (self.field_of_vision / 2.0).sin(),
                -(self.left_sensor.length * (self.field_of_vision / 2.0).sin()),
            ];
            let right_sensor = [
                0.0,
                0.0,
                self.right_sensor.length * (-self.field_of_vision / 2.0).cos(),
                // self.right_sensor.length * (-self.field_of_vision / 2.0).sin(),
                -(self.right_sensor.length * (-self.field_of_vision / 2.0).sin()),
            ];
            self.left_sensor.draw(c, g, next_angle);
            self.right_sensor.draw(c, g, next_angle);
            let square = ellipse::circle(self.x, self.y, self.radius); // 中心が(0,0)
            let mut color = if self.is_collide(arena) {
                [1.0, 1.0, 0.0, 1.0]
            }else {
                [0.0, 1.0, 0.0, 1.0]
            };
            if self.is_touched(arena) {
               color =[1.0, 0.0, 0.0, 1.0]
            };
            ellipse(color, square, c.transform, g);
              self.left_speed = 1.0;
              self.right_speed = 1.0;

            // center_line
            let center_line_color = [0.5, 0.5, 0.5, 1.0];
            let center = [self.x, self.y, self.x + self.radius, self.y];
            let zero_center = [0.0, 0.0, self.radius, 0.0];
            // let transed = c.transform.trans(self.x, self.y);
            // line(center_line_color, 1.0, [0.0, 0.0, self.radius, 0.0], start_point, g);
            // line(center_line_color, 1.0, center, c.transform.rot_deg(next_angle), g);
            // line(center_line_color, 1.0, center, c.transform.rot_deg(30.0), g);
            // 初期値じゃなくてtransで移さないとrotateの原点が移らない?
            line(center_line_color, 1.0, zero_center, transed.rot_deg(-next_angle), g);
            // line(center_line_color, 1.0, center, c.transform, g);

            // self.is_collide(arena);
            // println!("left_sensor data: {:?}", self.left_sensor.data(arena));
            // println!("right_sensor data: {:?}", self.right_sensor.data(arena));
            // self.x = self.x + trans_x;
            // self.y = self.y - trans_y;
            self.x = self.x - trans_x;
            self.y = self.y + trans_y;
            self.angle = next_angle;
            self.left_sensor.update((self.x, self.y), self.angle);
            self.right_sensor.update((self.x, self.y), self.angle);
            self.back = self.back - 1;
            println!("back!!!!!!!!!!!!: {}, trans_x: {}, trans_y: {}, self_x: {}, self_y: {}", self.back, trans_x, trans_y, self.x, self.y);
            return ;

















            }
if self.is_touched(&arena)  {
        self.back = 50;
        return;
}
            let start_point = c.transform.trans((150) as f64, (150) as f64);

            let v = (action.0 + action.1) / 2.0;
            let w = (action.1 - action.0) / (2.0 * self.radius);

            let l_l = action.0 * t;
            let l_r = action.1 * t;
            let l = (l_l + l_r) / 2.0; // delta-l
            // let delta_angle = (l_r - l_l) / (2.0 * self.radius); // dleta-theta
            let delta_angle = 90.0*(l_r -l_l) / (self.radius*3.14); // dleta-theta

            let next_x = v * self.angle.to_radians().cos();
            let next_y = v * self.angle.to_radians().sin();
            let next_angle_diff = w;
            // let next_angle_diff = 100.0;
            // let next_angle = self.angle + next_angle_diff * (t);
            let next_angle: f64;
            if self.angle + delta_angle > 360.0 {
                next_angle = self.angle + delta_angle - 360.0;
            } else {
                next_angle = self.angle + delta_angle;
            }
            // let trans_x: f64 = next_x * t * (next_angle * t / 2.0 + self.angle).to_radians().cos();
            // let trans_y: f64 = next_y * t * (next_angle * t / 2.0 + self.angle).to_radians().sin();
            let mut p = 0.0;
            let mut trans_x: f64 = 0.0;
                let mut trans_y: f64 = 0.0;
            if action.1 != action.0 {
              // p = self.radius * (action.0 + action.1) / (action.1 - action.0);
              p =  l * (360.0/ delta_angle) * (1.0/(2.0*3.14));
              let delta_l_dash = 2.0 * p * (delta_angle / 2.0).to_radians().sin();
                println!("delta_l: {}, delta_r: {}, delata_angle: {}, p: {}, delta_l :{}", action.0, action.1,delta_angle,  p, delta_l_dash);
              // trans_x = delta_l_dash * (self.angle + delta_l_dash / 2.0).to_radians().cos();
              // trans_y = delta_l_dash * (self.angle + delta_l_dash / 2.0).to_radians().sin();
              trans_x = delta_l_dash * (self.angle + delta_angle / 2.0).to_radians().cos();
              trans_y = delta_l_dash * (self.angle + delta_angle / 2.0).to_radians().sin();
            } else {
              // trans_x = l * (self.angle + delta_angle / 2.0).to_radians().cos();
              // trans_y = l * (self.angle + delta_angle / 2.0).to_radians().sin();
                println!("delta_l: {}, delta_r: {}, angle: {}, delata_angle: {}, next_x: {}, next_y: {}", action.0, action.1,self.angle, delta_angle, next_x, next_y);
              trans_x = next_x * t *(self.angle + delta_angle / 2.0).to_radians().cos();
              trans_y = next_y * t*(self.angle + delta_angle / 2.0).to_radians().sin();
            }
                // println!("trans_x: {}", trans_x);
                // thread::sleep(time::Duration::from_millis(100));
            // println!(
            //     "angle: {}, delta_angle: {}, next_angle_diff: {}, trans_x: {}, trans_y: {}, delta_l_dash: {}, p: {}, delta_angle: {}",
            //     self.angle, delta_angle, next_angle_diff, trans_x, trans_y, delta_l_dash , p , delta_angle
            // );

            let transed = c.transform.trans(self.x, self.y);
            // sensor
            let left_sensor = [
                0.0,
                0.0,
                self.left_sensor.length * (self.field_of_vision / 2.0).cos(),
                // self.left_sensor.length * (self.field_of_vision / 2.0).sin(),
                -(self.left_sensor.length * (self.field_of_vision / 2.0).sin()),
            ];
            let right_sensor = [
                0.0,
                0.0,
                self.right_sensor.length * (-self.field_of_vision / 2.0).cos(),
                // self.right_sensor.length * (-self.field_of_vision / 2.0).sin(),
                -(self.right_sensor.length * (-self.field_of_vision / 2.0).sin()),
            ];
            // println!("to_x: {}, to_y: {}, field_of_vision: {}", 
            //     self.left_sensor.length * (self.field_of_vision / 2.0).cos(),
            //     self.left_sensor.length * (self.field_of_vision / 2.0).sin(),
            //     self.field_of_vision 
            //          );
            self.left_sensor.draw(c, g, next_angle);
            self.right_sensor.draw(c, g, next_angle);
            // line(self.left_sensor.color, 1.0, left_sensor, transed.rot_deg(-next_angle), g);
            // line(self.right_sensor.color, 1.0, right_sensor, transed.rot_deg(-next_angle), g);
            // line(sensor_color, 1.0, left_sensor, start_point, g);
            // line(sensor_color, 1.0, right_sensor, start_point, g);
            // let right_sensor = [0.0, 0.0, self.radius * -cos1/2, self.radius * -sin 1/2];

            // eater
            let square = ellipse::circle(self.x, self.y, self.radius); // 中心が(0,0)
            let mut color = if self.is_collide(arena) {
                [1.0, 1.0, 0.0, 1.0]
            }else {
                [0.0, 1.0, 0.0, 1.0]
            };
            if self.is_touched(arena) {
               color =[1.0, 0.0, 0.0, 1.0]
            };
            // ellipse(red.clone(), square, start_point.trans(0.0, 0.0), g);
            // thread::sleep(time::Duration::from_millis(9000));
            ellipse(color, square, c.transform, g);
            if let Some(data) = self.left_sensor.data(arena) {
              self.left_speed = 1.0 + 2.0 * data;
              // self.left_speed = 2.0 + 2.0 * data;
              // self.left_speed = self.left_speed + 2.0 * data;
            }
            if let Some(data) = self.right_sensor.data(arena) {
              self.right_speed = 2.0 + 2.0 * data;
              // self.right_speed = self.right_speed + 2.0 * data;
                if data > 0.0 {

                // thread::sleep(time::Duration::from_millis(3000));
                }
            }
            // println!("{:?}", (self.left_speed, self.right_speed));

            // center_line
            let center_line_color = [0.5, 0.5, 0.5, 1.0];
            let center = [self.x, self.y, self.x + self.radius, self.y];
            let zero_center = [0.0, 0.0, self.radius, 0.0];
            // let transed = c.transform.trans(self.x, self.y);
            // line(center_line_color, 1.0, [0.0, 0.0, self.radius, 0.0], start_point, g);
            // line(center_line_color, 1.0, center, c.transform.rot_deg(next_angle), g);
            // line(center_line_color, 1.0, center, c.transform.rot_deg(30.0), g);
            // 初期値じゃなくてtransで移さないとrotateの原点が移らない?
            line(center_line_color, 1.0, zero_center, transed.rot_deg(-next_angle), g);
            // line(center_line_color, 1.0, center, c.transform, g);

            // self.is_collide(arena);
            // println!("left_sensor data: {:?}", self.left_sensor.data(arena));
            // println!("right_sensor data: {:?}", self.right_sensor.data(arena));
            self.x = self.x + trans_x;
            self.y = self.y - trans_y;
            self.angle = next_angle;
            self.left_sensor.update((self.x, self.y), self.angle);
            self.right_sensor.update((self.x, self.y), self.angle);
            // println!("x: {}, y: {}, next_angle: {}", self.x, self.y, next_angle);
        });
    }
    pub fn is_touched(&self, arena: &Arena) -> bool {
        let point = Point2::new(self.x, self.y);
        let distance = arena.cuboid.distance_to_point(&arena.transformed, &point, false);
        // println!("distance: {}", distance);
        -distance < (self.radius)
    }

    pub fn is_collide(&self, arena: &Arena) -> bool {
        // self.left_sensor.is_collide(arena) || self.right_sensor.is_collide(arena)
        println!("left collide");
        let l = self.left_sensor.is_collide(arena);
        println!("right collide");
        let r = self.right_sensor.is_collide(arena);
        l || r
        // self.left_sensor.is_collide(arena)
        // self.right_sensor.is_collide(arena)
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
        if let Some(i) = inter {
            let i_point = (self.x + dir_x * i.toi, y + dir_y * i.toi);
            let distance = ((self.x - i_point.0).powi(2) + (y - i_point.1).powi(2)).sqrt();
            // println!(
            //     "x: {}, y: {}, sub_y : {}, angle: {}, toi: {}, dir_x: {}, dir_y: {}, distance: {}, i_point: {:?}",
            //     self.x, self.y, y, self.angle, i.toi, dir_x, dir_y, distance, i_point
            // );
            // println!("i_point: {:?}", i_point);
            // thread::sleep(time::Duration::from_millis(9000));
            return Some(distance);
        }
        None
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
}

struct Feed {}
