//! 平行坐标图演示
//!
//! 展示多维数据可视化功能

use vizuara_core::Color;
use vizuara_plots::{ParallelAxis, ParallelCoordinates, ParallelSeries, PlotArea};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 平行坐标图演示 - Multi-dimensional Data Visualization");

    // 创建汽车性能对比平行坐标图
    let mut parallel = ParallelCoordinates::new().title("汽车性能多维对比");

    // 定义坐标轴
    let power_axis = ParallelAxis::new("马力".to_string(), 100.0, 600.0);
    let torque_axis = ParallelAxis::new("扭矩".to_string(), 200.0, 800.0);
    let fuel_axis = ParallelAxis::new("油耗".to_string(), 5.0, 15.0);
    let acceleration_axis = ParallelAxis::new("0-100km/h".to_string(), 3.0, 10.0);
    let price_axis = ParallelAxis::new("价格(万)".to_string(), 20.0, 200.0);

    parallel = parallel
        .add_axis(power_axis)
        .add_axis(torque_axis)
        .add_axis(fuel_axis)
        .add_axis(acceleration_axis)
        .add_axis(price_axis);

    // 添加汽车数据
    let bmw_m3 = ParallelSeries::new("BMW M3".to_string(), vec![480.0, 600.0, 10.2, 4.1, 85.0])
        .color(Color::rgb(0.2, 0.4, 0.8));

    let audi_rs5 = ParallelSeries::new("Audi RS5".to_string(), vec![450.0, 600.0, 9.8, 3.9, 90.0])
        .color(Color::rgb(0.8, 0.2, 0.2));

    let mercedes_c63 = ParallelSeries::new(
        "Mercedes C63".to_string(),
        vec![510.0, 700.0, 11.5, 4.0, 95.0],
    )
    .color(Color::rgb(0.2, 0.8, 0.2));

    let lexus_rcf = ParallelSeries::new(
        "Lexus RC F".to_string(),
        vec![472.0, 530.0, 12.1, 4.4, 75.0],
    )
    .color(Color::rgb(0.8, 0.6, 0.2));

    let tesla_model_s = ParallelSeries::new(
        "Tesla Model S".to_string(),
        vec![1020.0, 1050.0, 0.0, 2.1, 120.0],
    )
    .color(Color::rgb(0.6, 0.2, 0.8));

    parallel = parallel
        .add_series(bmw_m3)
        .add_series(audi_rs5)
        .add_series(mercedes_c63)
        .add_series(lexus_rcf)
        .add_series(tesla_model_s);

    // 设置样式
    parallel = parallel.axis_spacing(80.0).show_grid(true);

    let plot_area = PlotArea::new(80.0, 100.0, 640.0, 400.0);
    let scene = Scene::new(plot_area).add_parallel_coordinates(parallel);

    let figure = Figure::new(800.0, 600.0)
        .title("平行坐标图演示")
        .add_scene(scene);

    println!("✅ 平行坐标图演示图形已创建！");
    println!("这个图表展示了汽车多个性能指标的对比，每条线代表一款车型。");

    show_figure(figure)?;
    Ok(())
}
