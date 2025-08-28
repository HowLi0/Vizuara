//! 等高线图演示
//!
//! 展示3D数据的2D等高线可视化功能

use vizuara_plots::{ContourPlot, PlotArea};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️ 等高线图演示 - Contour Plot Visualization");

    // 生成3D数据 - 双高斯峰
    let width = 25;
    let height = 25;
    let mut data = vec![vec![0.0; width]; height];
    
    for i in 0..height {
        for j in 0..width {
            let x = (j as f32 - width as f32 / 2.0) / 4.0;
            let y = (i as f32 - height as f32 / 2.0) / 4.0;
            
            // 创建两个高斯峰
            let peak1 = (-((x - 1.5).powi(2) + (y - 1.0).powi(2))).exp() * 8.0;
            let peak2 = (-((x + 1.0).powi(2) + (y + 1.5).powi(2))).exp() * 6.0;
            let peak3 = (-((x).powi(2) + (y - 2.0).powi(2))).exp() * 4.0;
            
            data[i][j] = peak1 + peak2 + peak3;
        }
    }

    // 创建等高线图
    let x_values: Vec<f32> = (0..width).map(|i| i as f32).collect();
    let y_values: Vec<f32> = (0..height).map(|i| i as f32).collect();
    
    let contour_plot = ContourPlot::new()
        .from_grid(&x_values, &y_values, &data)
        .title("3D高度分布等高线图")
        .auto_levels(10)
        .filled(false)
        .show_labels(true, 10.0);

    let plot_area = PlotArea::new(80.0, 80.0, 640.0, 480.0);
    let scene = Scene::new(plot_area)
        .add_contour_plot(contour_plot);
    
    let figure = Figure::new(800.0, 640.0)
        .title("等高线图演示")
        .add_scene(scene);

    println!("✅ 等高线图演示图形已创建！");
    println!("这个图表展示了3D高度数据的等高线分布，每条线代表相同高度。");
    
    show_figure(figure)?;
    Ok(())
}
