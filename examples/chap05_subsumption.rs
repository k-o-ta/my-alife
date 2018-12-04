extern crate my_alife;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate piston_window;
extern crate rand;

use my_alife::simulator::module::{AvoidModule, ExploreModule, Module, WanderModule};
use my_alife::simulator::vehicle_simulator::*;

fn main() {
    let size = (600, 480);
    simulation(size);
}

fn simulation(size: (u32, u32)) {
    let mut _module = AvoidModule::new();
    let mut _module = WanderModule::new();
    let mut module = ExploreModule::new();
    Simulator::new(size).run(|eater_self, ref arena| {
        module.set_input(eater_self.sensor_data(arena));
        module.update();
        eater_self.left_speed = module.get_wheelspeed().0;
        eater_self.right_speed = module.get_wheelspeed().1;
        eater_self.update_color(module.get_color());
    });
}
