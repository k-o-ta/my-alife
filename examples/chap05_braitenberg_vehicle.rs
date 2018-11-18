extern crate my_alife;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate piston_window;
extern crate rand;

use my_alife::simulator::vehicle_simulator::*;
use na::{Isometry2, Point2, Vector2};
use ncollide2d::query::{Ray, RayCast, RayInterferencesCollector};
use ncollide2d::shape::{Ball, Cuboid};
use piston_window::*;
use rand::{thread_rng, Rng};
use std::sync::Arc;

fn main() {
    // eater();
    let size = (600, 480);
    // ray(size);
    ray_test();
    // eater(size);
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
fn ray_test() {
    println!("{}", 3.0_f64.powf(1.0 / 2.0));
    let ball = Ball::new(5.0);
    let cuboid = Cuboid::new(Vector2::new(20.0, 20.0));
    let ray_inside = Ray::new(Point2::new(10.0, 10.0), Vector2::y());
    let ray_inside2 = Ray::new(Point2::new(10.0, 10.0), Vector2::new(3.0_f64.powf(1.0 / 2.0), 1.0));
    let ray_inside3 = Ray::new(Point2::new(-10.0, 0.0), Vector2::x());
    let inter = cuboid
        .toi_and_normal_with_ray(&Isometry2::identity(), &ray_inside, false)
        .unwrap();
    println!("toi: {}, normal: {}", inter.toi, inter.normal);
    let inter2 = cuboid
        .toi_and_normal_with_ray(&Isometry2::identity(), &ray_inside2, false)
        .unwrap();
    println!("toi: {}, normal: {}", inter2.toi, inter2.normal);
    let inter3 = ball
        .toi_and_normal_with_ray(&Isometry2::identity(), &ray_inside3, false)
        .unwrap();
    println!("toi3: {}, normal3: {}", inter3.toi, inter3.normal);
}

fn ray(size: (u32, u32)) {
    // let vec = Vector2::new(1.0, 2.0);
    let y = Vector2::y();
    println!("v1: {}, v2: {}", Vector2::new(3.0, 1.0), y);
    let _ray = Ray::new(Point2::new(150.0, 150.0), y);
    let vec = Vector2::new(1.0, 1.0);
    let hit_ray = Ray::new(Point2::new(150.0, 150.0), vec);
    let arena = Arena::new(size.0 as f64, size.1 as f64);
    let intersection = arena
        .cuboid
        .toi_and_normal_with_ray(&arena.transformed, &hit_ray, false)
        .unwrap();
    // .toi_and_normal_with_ray(&iso, &hit_ray, false);
    //
    println!("toi: {}, normal: {}", intersection.toi, intersection.normal);

    // println!("point2: {}, vec: {}", Point2::new(2.0, 2.0), vec);
}

fn eater(size: (u32, u32)) {
    let mut arena = Arena::new(size.0 as f64, size.1 as f64);
    let mut eater = Eater::new((100.0, 100.0), size.1 as f64);
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut t = 0;
    let mut rng = thread_rng();
    let speed_l = rng.gen_range(300, 500) as f64;
    let speed_r = rng.gen_range(100, 200);
    while let Some(e) = window.next() {
        // eater.render(&mut window, &e, (150.0, 450.0));
        // eater.render(
        //     &mut window,
        //     &e,
        //     (rng.gen_range(100, 900) as f64, rng.gen_range(300, 500) as f64), // 直進のときだけ動きが速い
        //     // (200 as f64, 200 as f64),
        //     &mut arena,
        // );
    }
}
fn run() {
    // let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (1200, 900))
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (400, 300))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut t = 0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            // let center = c.transform.trans((100 / 2) as f64, (100 / 2) as f64);
            let center = c.transform.trans((100) as f64, (100) as f64);
            // let square = ellipse::circle(50.0, 50.0, 50.0);
            let center2 = c.transform.trans((150) as f64, (150) as f64);
            let square2 = ellipse::circle(50.0, 50.0, 50.0);
            let center3 = c.transform.trans((200) as f64, (200) as f64);
            let square3 = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            let square = rectangle::square(0.0, 0.0, 100.0);
            // let corner = c.transform;
            let corner = c
                .transform
                .trans((0) as f64, (50) as f64)
                .trans((0) as f64, (-50) as f64);
            let blue = [0.0, 0.0, 1.0, 1.0];
            // circle_arc(
            //     red.clone(),
            //     10.0,
            //     10.0,
            //     10.0,
            //     [10.0, 10.0, 10.0, 10.0],
            //     center.rot_deg(t as f64),
            //     g,
            // );
            // ellipse(
            //     red.clone(),
            //     [20.0, 20.0, 20.0, 20.0],
            //     center.rot_deg(t as f64).trans(-10.0, -10.0),
            //     g,
            // );
            // ellipse(
            //     red.clone(),
            //     ellipse::circle(100.0, 100.0, 50.0),
            //     center2.rot_deg(t as f64).trans(100.0, 100.0),
            //     g,
            // );
            // rectangle(red, square, center.rot_rad(t as f64).trans(t as f64, t as f64), g);
            // rectangle(red, square, center.rot_deg(t as f64).trans(0.0, 0.0), g);
            rectangle(blue, square, corner, g);
            ellipse(
                red.clone(),
                square2,
                center2.rot_deg(t as f64).trans(-50.0, -50.0).trans(0.0, 0.0),
                g,
            );
            // ellipse(
            //     red.clone(),
            //     square,
            //     center2.rot_deg(t as f64).trans(-50.0, -50.0).trans(0.0, 0.0),
            //     g,
            // );
            // rectangle(red, square, center.rot_deg(t as f64).trans(-50.0, -50.0), g);
            let line_shape = Line::new([0.0; 4], 3.0);
            // .color(red).radius(3.0).shape(Shape::Round);
            // line([1.0; 4], 3.0, line_shape, center, g);
            let center4 = c.transform.trans((300) as f64, (300) as f64);
            line([1.0; 4], 1.0, [0.0, 0.0, 50.0, 0.0], center2.rot_deg(t as f64), g);
            // Line::draw(&lien_shape, 3.0, line_shape, center, g);
            rectangle(red, square3, center3.rot_deg(t as f64).trans(-50.0, -50.0), g);
        });
        t = (t + 1) % 360;
    }
}

// fn main() {
//     let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [1200, 900])
//         .exit_on_esc(true)
//         .build()
//         .unwrap();
//     while let Some(event) = window.next() {
//         window.draw_2d(&event, |context, graphics| {
//             clear([1.0; 4], graphics);
//             rectangle(
//                 [1.0, 0.0, 0.0, 1.0], // red
//                 [0.0, 0.0, 50.0, 50.0],
//                 context.transform,
//                 graphics,
//             );
//         });
//     }
// }
