extern crate my_alife;
extern crate piston_window;
use my_alife::simulator::vehicle_simulator::*;
use piston_window::*;
fn main() {
    eater();
}

fn eater() {
    let mut eater = Eater::new();
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (1200, 900))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut t = 0;
    while let Some(e) = window.next() {
        eater.render(&mut window, &e);
    }
}
fn run() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (1200, 900))
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
