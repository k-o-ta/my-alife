extern crate my_alife;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate piston_window;
extern crate rand;

use my_alife::simulator::module::{AvoidModule, Module, WanderModule};
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
    let mut module = AvoidModule::new();
    let mut module = WanderModule::new();
    Simulator::new(size).run(|eater_self, ref arena| {
        module.set_input((
            eater_self.left_sensor.data(arena).unwrap_or(0.0),
            eater_self.right_sensor.data(arena).unwrap_or(0.0),
        ));
        module.update();
        eater_self.left_speed = module.get_wheelspeed().0;
        eater_self.right_speed = module.get_wheelspeed().1;
    });
}
