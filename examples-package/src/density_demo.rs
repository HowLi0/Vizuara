//! 密度图演示
//!
//! 展示核密度估计可视化功能

use vizuara_plots::{DensityPlot, KernelType, PlotArea};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;
use vizuara_core::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 密度图演示 - Kernel Density Estimation");

    // 生成示例数据 - 混合正态分布
    let mut data = Vec::new();
    
    // 第一个峰值 (均值=2, 标准差=0.8)
    for _ in 0..150 {
        let value = 2.0 + (rand::random::<f32>() - 0.5) * 1.6;
        data.push(value);
    }
    
    // 第二个峰值 (均值=6, 标准差=1.2)
    for _ in 0..100 {
        let value = 6.0 + (rand::random::<f32>() - 0.5) * 2.4;
        data.push(value);
    }

    // 创建密度图
    let density_plot = DensityPlot::new()
        .data(&data)
        .title("双峰数据分布")
        .kernel(KernelType::Gaussian)
        .bandwidth(0.3)
        .resolution(300)
        .line_color(Color::rgb(0.1, 0.5, 0.8))
        .line_width(2.0)
        .fill_color(Some(Color::rgba(0.3, 0.7, 0.9, 0.6)))
        .show_points(true, 1.5);

    let plot_area = PlotArea::new(80.0, 80.0, 640.0, 440.0);
    let scene = Scene::new(plot_area)
        .add_density_plot(density_plot);
    
    let figure = Figure::new(800.0, 600.0)
        .title("密度图演示")
        .add_scene(scene);

    println!("✅ 密度图演示图形已创建！");
    println!("这个图表展示了数据的概率密度分布，可以清楚地看到两个峰值。");
    
    show_figure(figure)?;
    Ok(())
}
