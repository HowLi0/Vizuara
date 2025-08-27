//! LinePlot 演示：展示折线图功能
//! 
//! 这个示例展示了：
//! - 折线图的创建和样式设置
//! - 散点图和折线图的组合显示
//! - 多种线条样式

use vizuara_core::{LinearScale, Color};
use vizuara_plots::{ScatterPlot, LinePlot, PlotArea};
use vizuara_scene::{Scene, Figure};
use vizuara_window::show_figure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 LinePlot 演示启动");
    println!("🔗 展示折线图和散点图组合...");
    
    // 1. 创建测试数据 - 模拟函数 y = sin(x) + noise
    let mut data_points = Vec::new();
    let mut line_data = Vec::new();
    
    for i in 0..=20 {
        let x = i as f32 * 0.5; // 0 到 10
        let y_line = (x * 0.5).sin() * 3.0 + 5.0; // 平滑的sin函数
        let y_scatter = y_line + (i as f32 * 0.3).sin() * 0.5; // 添加一些噪声
        
        line_data.push((x, y_line));
        data_points.push((x, y_scatter));
    }
    
    println!("✅ 创建了 {} 个数据点", data_points.len());
    
    // 2. 创建折线图 - 显示趋势线
    let line_plot = LinePlot::new()
        .data(&line_data)
        .color(Color::rgb(0.8, 0.2, 0.2))  // 红色线条
        .line_width(3.0)
        .auto_scale();
    
    // 3. 创建散点图 - 显示实际数据点
    let scatter_plot = ScatterPlot::new()
        .data(&data_points)
        .color(Color::rgb(0.2, 0.4, 0.8))  // 蓝色点
        .size(4.0)
        .auto_scale();
    
    // 4. 设置坐标轴
    let x_scale = LinearScale::new(0.0, 10.0);
    let y_scale = LinearScale::new(0.0, 10.0);
    
    // 5. 创建场景 - 组合折线图和散点图
    let plot_area = PlotArea::new(80.0, 80.0, 640.0, 440.0);
    let scene = Scene::new(plot_area)
        .add_x_axis(x_scale, Some("X 值".to_string()))
        .add_y_axis(y_scale, Some("Y 值".to_string()))
        .add_line_plot(line_plot)          // 先添加线条
        .add_scatter_plot(scatter_plot)    // 再添加点，显示在线条上方
        .title("折线图 + 散点图组合");
    
    // 6. 创建 Figure
    let figure = Figure::new(800.0, 600.0)
        .title("LinePlot 演示 - 趋势线与数据点")
        .add_scene(scene);
    
    println!("🎨 Figure 创建完成，开始渲染窗口...");
    println!("💡 红色线条显示趋势，蓝色点显示实际数据");
    println!("💡 按 ESC 退出，按 R 刷新");
    
    // 7. 显示在窗口中
    show_figure(figure)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_demo_data_creation() {
        let mut line_data = Vec::new();
        
        for i in 0..=10 {
            let x = i as f32 * 0.5;
            let y = (x * 0.5).sin() * 3.0 + 5.0;
            line_data.push((x, y));
        }
        
        let line_plot = LinePlot::new()
            .data(&line_data)
            .color(Color::rgb(0.8, 0.2, 0.2))
            .auto_scale();
        
        assert_eq!(line_plot.data_len(), 11);
        
        let bounds = line_plot.data_bounds().unwrap();
        assert_eq!(bounds.0.x, 0.0);
        assert_eq!(bounds.1.x, 5.0);
    }

    #[test]
    fn test_combined_scene() {
        let line_data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 1.0)];
        let scatter_data = vec![(1.1, 2.1), (2.1, 3.1), (3.1, 1.1)];
        
        let line_plot = LinePlot::new()
            .data(&line_data)
            .color(Color::rgb(1.0, 0.0, 0.0))
            .auto_scale();
            
        let scatter_plot = ScatterPlot::new()
            .data(&scatter_data)
            .color(Color::rgb(0.0, 0.0, 1.0))
            .auto_scale();
        
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let scene = Scene::new(plot_area)
            .add_line_plot(line_plot)
            .add_scatter_plot(scatter_plot);
        
        let primitives = scene.generate_primitives();
        assert!(!primitives.is_empty());
        
        // 应该包含线条和点的图元
        let line_strips = primitives.iter().filter(|p| matches!(p, vizuara_core::Primitive::LineStrip(_))).count();
        let points = primitives.iter().filter(|p| matches!(p, vizuara_core::Primitive::Points(_))).count();
        
        assert!(line_strips > 0, "应该有线条图元");
        assert!(points > 0, "应该有点图元");
    }
}
