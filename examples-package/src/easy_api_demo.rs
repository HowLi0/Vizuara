use vizuara_easy::prelude::*;
use vizuara_core::Color;

fn main() {
    let data: Vec<(f32,f32)> = (0..100).map(|i| {
        let x = i as f32 * 0.1;
        (x, x.sin())
    }).collect();

    let mut fig = figure(800.0, 600.0);
    let sparse: Vec<(f32,f32)> = data.iter().cloned().step_by(10).collect();
    fig.title("Easy API Demo")
        .subplot_full()
        .plot(&data, Color::rgb(0.2,0.6,1.0), 2.0)
        .scatter(&sparse, Color::rgb(1.0,0.3,0.3), 6.0);

    if let Err(e) = fig.show() { eprintln!("Error: {:?}", e); }
}
