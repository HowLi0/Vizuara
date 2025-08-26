//! 阶段2第一个完整示例：真实窗口中的散点图
//! 
//! 这个示例展示了从数据到窗口显示的完整流程

use vizuara_core::{LinearScale, Color};
use vizuara_plots::{ScatterPlot, PlotArea};
use vizuara_scene::{Scene, Figure};
use vizuara_window::show_figure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 阶段2 - 完整可视化示例启动");
    println!("📊 创建真实的散点图窗口应用...");
    
    // 1. 创建更丰富的测试数据
    let data = vec![
        (1.0, 2.1), (1.2, 2.5), (1.4, 2.8), (1.6, 3.1),
        (1.8, 3.5), (2.0, 3.8), (2.2, 4.1), (2.4, 4.5),
        (2.6, 4.8), (2.8, 5.1), (3.0, 5.5), (3.2, 5.8),
        (3.4, 6.1), (3.6, 6.5), (3.8, 6.8), (4.0, 7.1),
        (4.2, 7.5), (4.4, 7.8), (4.6, 8.1), (4.8, 8.5),
        (5.0, 8.8), (5.2, 9.1), (5.4, 9.5), (5.6, 9.8),
    ];
    
    println!("✅ 创建了 {} 个数据点", data.len());
    
    // 2. 创建散点图
    let scatter = ScatterPlot::new()
        .data(&data)
        .color(Color::rgb(0.8, 0.2, 0.4))  // 深粉色
        .size(6.0)
        .auto_scale();
    
    // 3. 设置坐标轴
    let x_scale = LinearScale::new(0.0, 6.0);
    let y_scale = LinearScale::new(0.0, 10.0);
    
    // 4. 创建场景（更大的绘图区域）
    let plot_area = PlotArea::new(80.0, 80.0, 640.0, 440.0);
    let scene = Scene::new(plot_area)
        .add_x_axis(x_scale, Some("时间 (秒)".to_string()))
        .add_y_axis(y_scale, Some("温度 (°C)".to_string()))
        .add_scatter_plot(scatter)
        .title("温度变化散点图");
    
    // 5. 创建 Figure
    let figure = Figure::new(800.0, 600.0)
        .title("Vizuara 阶段2 - 实时可视化演示")
        .add_scene(scene);
    
    println!("🎨 Figure 创建完成，开始渲染窗口...");
    println!("💡 提示：按 ESC 退出，按 R 刷新");
    
    // 6. 显示在窗口中
    show_figure(figure).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_demo_data_creation() {
        let data = vec![
            (1.0, 2.1), (1.2, 2.5), (1.4, 2.8), (1.6, 3.1),
            (1.8, 3.5), (2.0, 3.8), (2.2, 4.1), (2.4, 4.5),
        ];
        
        let scatter = ScatterPlot::new()
            .data(&data)
            .color(Color::rgb(0.8, 0.2, 0.4))
            .auto_scale();
        
        assert_eq!(scatter.data_len(), 8);
        
        let bounds = scatter.data_bounds().unwrap();
        assert_eq!(bounds.0.x, 1.0);
        assert_eq!(bounds.1.x, 2.4);
    }
}
