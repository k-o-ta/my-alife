extern crate my_alife;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate piston_window;
extern crate rand;

use my_alife::simulator::module::{AvoidModule, Module};
use my_alife::simulator::vehicle_simulator::*;
use na::{Isometry2, Point2, Vector2};
use ncollide2d::query::{Ray, RayCast, RayInterferencesCollector};
use ncollide2d::shape::{Ball, Cuboid};
use piston_window::*;
use rand::{thread_rng, Rng};
use std::sync::Arc;

fn main() {
    let size = (600, 480);
    simulation(size);
}

fn simulation(size: (u32, u32)) {
    let mut avoid_module = AvoidModule::new();
    Simulator::new(size).run(|eater_self, ref arena| {
        // let hoge = eater_self.left_sensor.data(arena).unwrap();
        // let hoge = eater_self.right_sensor.data(arena).unwrap();
        avoid_module.set_input((
            eater_self.left_sensor.data(arena).unwrap_or(0.0),
            eater_self.right_sensor.data(arena).unwrap_or(0.0),
        ));
        avoid_module.update();
        eater_self.left_speed = avoid_module.get_wheelspeed().0;
        eater_self.right_speed = avoid_module.get_wheelspeed().1;
    });
}
