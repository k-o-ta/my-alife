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
