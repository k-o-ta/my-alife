use piston_window::*;
// gfx_graphics::back_end
pub struct Eater {
    radius: f64,
    x: f64,
    y: f64,
    field_of_vision: f64,
    sensor_length: f64,
    left_speed: f64,
    right_speed: f64,
    angle: f64,
    next_angle: f64,
}

use std::f64;
impl Eater {
    pub fn new() -> Eater {
        Eater {
            radius: 50.0,
            field_of_vision: 120.0_f64.to_radians(),
            sensor_length: 50.0 * 2.5,
            x: 0.0,
            y: 600.0,
            left_speed: 15.0,
            right_speed: 45.0,
            angle: 0.0,
            next_angle: 0.0,
        }
    }
    pub fn render<E>(&mut self, w: &mut PistonWindow, e: &E)
    where
        E: generic_event::GenericEvent,
    {
        w.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            let t = 1.0;
            let start_point = c.transform.trans((150) as f64, (150) as f64);

            let v = (self.left_speed + self.right_speed) / 2.0;
            let w = (self.right_speed - self.left_speed) / (2.0 * self.radius);
            let p = self.radius * (self.left_speed + self.right_speed) / (self.right_speed - self.left_speed);

            let l_l = self.left_speed * t;
            let l_r = self.right_speed * t;
            let l = (l_l + l_r) / 2.0;
            let delta_angle = (l_r - l_l) / (2.0 * self.radius);

            let next_x = v * self.angle.to_radians().cos();
            let next_y = v * self.angle.to_radians().sin();
            let next_angle_diff = w;
            // let next_angle_diff = 100.0;
            // let next_angle = self.angle + next_angle_diff * (t);
            let next_angle = self.angle + delta_angle;
            // let trans_x: f64 = next_x * t * (next_angle * t / 2.0 + self.angle).to_radians().cos();
            // let trans_y: f64 = next_y * t * (next_angle * t / 2.0 + self.angle).to_radians().sin();
            let delta_l_dash = 2.0 * p * (delta_angle / 2.0).to_radians().sin();
            let trans_x: f64 = delta_l_dash * (self.angle + delta_l_dash / 2.0).to_radians().cos();
            let trans_y: f64 = delta_l_dash * (self.angle + delta_l_dash / 2.0).to_radians().sin();
            println!(
                "angle: {}, delta_angle: {}, next_angle_diff: {}, trans_x: {}, trans_y: {}",
                self.angle, delta_angle, next_angle_diff, trans_x, trans_y
            );

            // sensor
            let sensor_color = [0.5, 0.5, 0.5, 1.0];
            let left_sensor = [
                0.0,
                0.0,
                self.sensor_length * (self.field_of_vision / 2.0).cos(),
                self.sensor_length * (self.field_of_vision / 2.0).sin(),
            ];
            // println!(
            //     "radi: {}, cos: {}, sin: {}, {:?}",
            //     self.field_of_vision / 2.0,
            //     (self.field_of_vision / 2.0).cos(),
            //     (self.field_of_vision / 2.0).sin(),
            //     left_sensor
            // );
            let right_sensor = [
                0.0,
                0.0,
                self.sensor_length * (-self.field_of_vision / 2.0).cos(),
                self.sensor_length * (-self.field_of_vision / 2.0).sin(),
            ];
            line(sensor_color, 1.0, left_sensor, start_point, g);
            line(sensor_color, 1.0, right_sensor, start_point, g);
            // let right_sensor = [0.0, 0.0, self.radius * -cos1/2, self.radius * -sin 1/2];

            // eater
            let square = ellipse::circle(self.x, self.y, self.radius); // 中心が(0,0)
            let red = [1.0, 0.0, 0.0, 1.0];
            // ellipse(red.clone(), square, start_point.trans(0.0, 0.0), g);
            ellipse(red.clone(), square, c.transform, g);

            // center_line
            let center_line_color = [0.5, 0.5, 0.5, 1.0];
            line(center_line_color, 1.0, [0.0, 0.0, self.radius, 0.0], start_point, g);

            self.x = self.x + trans_x;
            self.y = self.y - trans_y;
            self.angle = next_angle;
            println!("x: {}, y: {}", self.x, self.y);
        });
    }
}
