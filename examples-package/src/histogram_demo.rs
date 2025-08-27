use vizuara_plots::{Histogram, BinningStrategy, PlotArea};
use vizuara_scene::{Scene, Figure};
use vizuara_core::{LinearScale, Color};
use vizuara_window::show_figure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Vizuara 直方图演示启动！");
    
    // 1. 生成随机数据（模拟正态分布）
    let mut data = Vec::new();
    for i in 0..1000 {
        let x = (i as f32 - 500.0) / 100.0; // -5 到 +5
        // 简单的正态分布近似
        let y = (-x * x / 2.0).exp() + (rand::random::<f32>() - 0.5) * 0.1;
        if rand::random::<f32>() < y {
            data.push(x);
        }
    }
    
    // 添加一些额外的数据点以获得更好的分布
    for _ in 0..500 {
        let x = (rand::random::<f32>() - 0.5) * 6.0; // -3 到 +3
        data.push(x);
    }
    
    println!("📈 生成了 {} 个数据点", data.len());
    
    // 2. 创建三种不同的直方图
    
    // 自动分桶的直方图
    let hist_auto = Histogram::new()
        .data(&data)
        .binning(BinningStrategy::Auto)
        .fill_color(Color::rgb(0.3, 0.7, 0.9))
        .stroke_color(Color::rgb(0.1, 0.3, 0.5))
        .auto_scale();
    
    println!("🔄 自动分桶: {} 个桶", hist_auto.bins().len());
    
    // 固定桶数的直方图
    let hist_fixed = Histogram::new()
        .data(&data)
        .binning(BinningStrategy::FixedCount(20))
        .fill_color(Color::rgb(0.9, 0.3, 0.3))
        .stroke_color(Color::rgb(0.5, 0.1, 0.1))
        .auto_scale();
    
    println!("📌 固定20桶: {} 个桶", hist_fixed.bins().len());
    
    // 固定宽度的直方图
    let hist_width = Histogram::new()
        .data(&data)
        .binning(BinningStrategy::FixedWidth(0.5))
        .fill_color(Color::rgb(0.3, 0.9, 0.3))
        .stroke_color(Color::rgb(0.1, 0.5, 0.1))
        .auto_scale();
    
    println!("📏 固定宽度0.5: {} 个桶", hist_width.bins().len());
    
    // 3. 创建场景并添加直方图 (只显示自动分桶的)
    let plot_area = PlotArea::new(80.0, 60.0, 640.0, 480.0);
    
    let scene = Scene::new(plot_area)
        .title("数据分布直方图 - 自动分桶")
        .add_histogram(hist_auto)
        .add_x_axis(LinearScale::new(-4.0, 4.0), Some("数值".to_string()))
        .add_y_axis(LinearScale::new(0.0, 50.0), Some("频次".to_string()));
    
    // 4. 创建图形并显示
    let figure = Figure::new(800.0, 600.0)
        .title("Vizuara 直方图演示")
        .add_scene(scene);
    
    println!("🚀 启动窗口渲染...");
    println!("💡 提示: ESC 退出, R 刷新");
    
    show_figure(figure)?;
    
    println!("✅ 演示完成!");
    Ok(())
}

// 简单的随机数生成 (用于演示)
mod rand {
    use std::cell::RefCell;
    
    thread_local! {
        static RNG_STATE: RefCell<u64> = RefCell::new(1);
    }
    
    pub fn random<T>() -> T
    where
        T: From<f32>,
    {
        RNG_STATE.with(|state| {
            let mut s = state.borrow_mut();
            *s = (*s).wrapping_mul(1664525).wrapping_add(1013904223);
            let normalized = (*s as f32) / (u64::MAX as f32);
            T::from(normalized)
        })
    }
}
