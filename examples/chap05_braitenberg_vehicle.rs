extern crate my_alife;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate piston_window;
extern crate rand;

use my_alife::simulator::vehicle_simulator::*;

fn main() {
    let size = (600, 480);
    simulation(size);
}

fn simulation(size: (u32, u32)) {
    Simulator::new(size).run(|eater_self, ref arena| {
        if let Some(data) = eater_self.left_sensor.data(arena) {
            eater_self.left_speed = 2.0 + 2.0 * data;
        }
        if let Some(data) = eater_self.right_sensor.data(arena) {
            eater_self.right_speed = 2.0 + 2.0 * data;
        }
    });
}
