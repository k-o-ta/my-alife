use rand::{thread_rng, Rng};
use simulator::vehicle_simulator::Color;
pub trait Module {
    fn set_input(&mut self, sensor_data: ((f64, f64), bool));
    fn update(&mut self);
    fn get_wheelspeed(&self) -> (f64, f64);
    fn get_color(&self) -> Color;
}

pub struct AvoidModule {
    left_distance: f64,
    righ_distance: f64,
    left_speed: f64,
    right_speed: f64,
    color: Color,
}

impl Module for AvoidModule {
    fn set_input(&mut self, sensor_data: ((f64, f64), bool)) {
        let distance = sensor_data.0;
        self.left_distance = distance.0;
        self.righ_distance = distance.1;
    }
    fn update(&mut self) {
        self.left_speed = 1.0 + 3.0 * self.left_distance;
        self.right_speed = 1.0 + 3.0 * self.righ_distance;
    }
    fn get_wheelspeed(&self) -> (f64, f64) {
        (self.left_speed, self.right_speed)
    }
    fn get_color(&self) -> Color {
        self.color
    }
}

impl AvoidModule {
    pub fn new() -> AvoidModule {
        AvoidModule {
            left_distance: 0.0,
            righ_distance: 0.0,
            left_speed: 0.0,
            right_speed: 0.0,
            color: Color::Red,
        }
    }
}

pub struct WanderModule {
    left_distance: f64,
    righ_distance: f64,
    left_speed: f64,
    right_speed: f64,
    color: Color,
    counter: u32,
    child_module: AvoidModule,
}

impl Module for WanderModule {
    fn set_input(&mut self, sensor_data: ((f64, f64), bool)) {
        let distance = sensor_data.0;
        self.left_distance = distance.0;
        self.righ_distance = distance.1;
        self.child_module.set_input(sensor_data);
    }
    fn update(&mut self) {
        if self.left_distance < 0.001 && self.righ_distance < 0.001 {
            self.counter = (self.counter + 1) % Self::TURN_END_STEP
        } else {
            self.counter = 0;
        }

        if self.counter < Self::TURN_START_STEP {
            self.child_module.update();
            self.left_speed = self.child_module.get_wheelspeed().0;
            self.right_speed = self.child_module.get_wheelspeed().1;
            self.color = self.child_module.get_color();
        // println!("avoiding, left: {}, right: {}", self.left_speed, self.right_speed);
        } else if self.counter == Self::TURN_START_STEP {
            println!("into wander");
            let random: f64 = thread_rng().gen();
            if random < 0.5 {
                self.left_speed = 1.5;
                self.right_speed = 1.0;
            } else {
                self.left_speed = 1.0;
                self.right_speed = 1.5;
            }
            self.color = Color::Green;
        }
    }
    fn get_wheelspeed(&self) -> (f64, f64) {
        (self.left_speed, self.right_speed)
    }
    fn get_color(&self) -> Color {
        self.color
    }
}

impl WanderModule {
    const TURN_START_STEP: u32 = 20;
    const TURN_END_STEP: u32 = 40;
    pub fn new() -> WanderModule {
        let left_distance = 0.0;
        let righ_distance = 0.0;
        let left_speed = 0.0;
        let right_speed = 0.0;
        let avoid_module = AvoidModule::new();
        WanderModule {
            left_distance,
            righ_distance,
            left_speed,
            right_speed,
            color: Color::Green,
            counter: 0,
            child_module: avoid_module,
        }
    }
}

pub struct ExploreModule {
    left_distance: f64,
    right_distance: f64,
    left_speed: f64,
    right_speed: f64,
    color: Color,
    touching: bool,
    child_module: WanderModule,
}

impl Module for ExploreModule {
    fn set_input(&mut self, sensor_data: ((f64, f64), bool)) {
        self.child_module.set_input(sensor_data);
        let distance = sensor_data.0;
        let is_touching = sensor_data.1;
        self.left_distance = distance.0;
        self.right_distance = distance.1;
        self.touching = is_touching;
    }
    fn update(&mut self) {
        if self.touching {
            self.right_speed = 0.0;
            self.left_speed = 0.0;
            self.color = Color::Blue;
        } else {
            self.child_module.update();

            self.left_speed = self.child_module.get_wheelspeed().0;
            self.right_speed = self.child_module.get_wheelspeed().1;
            self.color = self.child_module.get_color();
        }
    }
    fn get_wheelspeed(&self) -> (f64, f64) {
        (self.left_speed, self.right_speed)
    }
    fn get_color(&self) -> Color {
        self.color
    }
}

impl ExploreModule {
    pub fn new() -> ExploreModule {
        ExploreModule {
            left_distance: 0.0,
            right_distance: 0.0,
            left_speed: 0.0,
            right_speed: 0.0,
            color: Color::Blue,
            touching: false,
            child_module: WanderModule::new(),
        }
    }
}
