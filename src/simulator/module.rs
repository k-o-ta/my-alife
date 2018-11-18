use rand::{thread_rng, Rng};
pub trait Module {
    fn set_input(&mut self, distance: (f64, f64));
    fn update(&mut self);
    fn get_wheelspeed(&self) -> (f64, f64);
}

pub struct AvoidModule {
    left_distance: f64,
    righ_distance: f64,
    left_speed: f64,
    right_speed: f64,
}

impl Module for AvoidModule {
    fn set_input(&mut self, distance: (f64, f64)) {
        self.left_distance = distance.0;
        self.righ_distance = distance.1;
    }

    fn update(&mut self) {
        self.left_speed = 2.0 + 2.0 * self.left_distance;
        self.right_speed = 2.0 + 2.0 * self.righ_distance;
    }
    fn get_wheelspeed(&self) -> (f64, f64) {
        (self.left_speed, self.right_speed)
    }
}

impl AvoidModule {
    pub fn new() -> AvoidModule {
        AvoidModule {
            left_distance: 0.0,
            righ_distance: 0.0,
            left_speed: 0.0,
            right_speed: 0.0,
        }
    }
}

pub struct WanderModule {
    left_distance: f64,
    righ_distance: f64,
    left_speed: f64,
    right_speed: f64,
    counter: u32,
    child_module: AvoidModule,
}

impl Module for WanderModule {
    fn set_input(&mut self, distance: (f64, f64)) {
        self.left_distance = distance.0;
        self.righ_distance = distance.1;
    }
    fn update(&mut self) {
        if self.left_distance < 0.001 && self.righ_distance < 0.001 {
            self.counter = (self.counter + 1) % Self::TURN_END_STEP
        } else {
            self.counter = 0;
        }

        if self.counter < Self::TURN_START_STEP {
            self.child_module.set_input((self.left_distance, self.righ_distance));
            self.child_module.update();
            self.left_speed = self.child_module.get_wheelspeed().0;
            self.right_speed = self.child_module.get_wheelspeed().1;
        } else if self.counter == Self::TURN_START_STEP {
            let random: f64 = thread_rng().gen();
            if random < 0.5 {
                self.left_speed = 1.5;
                self.right_speed = 1.0;
            } else {
                self.left_speed = 1.0;
                self.right_speed = 1.5;
            }
        }
    }
    fn get_wheelspeed(&self) -> (f64, f64) {
        (self.left_speed, self.right_speed)
    }
}

impl WanderModule {
    const TURN_START_STEP: u32 = 100;
    const TURN_END_STEP: u32 = 180;
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
            counter: 0,
            child_module: avoid_module,
        }
    }
}
