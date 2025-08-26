//! 柱状图演示程序
//! 
//! 展示 Vizuara 的柱状图功能，包括基础柱状图和样式配置

use vizuara_core::{LinearScale, Color};
use vizuara_plots::{BarPlot, PlotArea};
use vizuara_scene::{Scene, Figure};
use vizuara_window::show_figure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 柱状图演示启动");
    println!("🎨 创建多种柱状图样式...");
    
    // 1. 创建销售数据
    let sales_data = [
        ("一月", 85.2),
        ("二月", 92.1),
        ("三月", 78.5),
        ("四月", 105.3),
        ("五月", 98.7),
        ("六月", 112.4),
    ];
    
    println!("✅ 创建了 {} 个月的销售数据", sales_data.len());
    
    // 2. 创建基础柱状图
    let bar_chart = BarPlot::new()
        .data(&sales_data)
        .fill_color(Color::rgb(0.2, 0.6, 0.9))  // 蓝色
        .stroke(Color::rgb(0.1, 0.3, 0.7), 1.5)
        .bar_width(0.7)
        .title("月度销售额")
        .auto_scale();
    
    // 3. 设置坐标轴
    let y_scale = LinearScale::new(0.0, 120.0);
    
    // 4. 创建场景
    let plot_area = PlotArea::new(80.0, 80.0, 640.0, 440.0);
    let scene = Scene::new(plot_area)
        .add_y_axis(y_scale, Some("销售额 (万元)".to_string()))
        .add_bar_plot(bar_chart)
        .title("2025年上半年销售统计");
    
    // 5. 创建 Figure
    let figure = Figure::new(800.0, 600.0)
        .title("Vizuara 柱状图演示")
        .add_scene(scene);
    
    println!("🎨 柱状图创建完成，开始渲染...");
    println!("💡 提示：按 ESC 退出，按 R 刷新");
    
    // 6. 显示在窗口中
    show_figure(figure).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_demo_data_creation() {
        let sales_data = [("Jan", 100.0), ("Feb", 120.0), ("Mar", 90.0)];
        
        let bar_chart = BarPlot::new()
            .data(&sales_data)
            .fill_color(Color::rgb(0.2, 0.6, 0.9))
            .auto_scale();
        
        assert_eq!(bar_chart.data_len(), 3);
        assert_eq!(bar_chart.categories(), vec!["Jan", "Feb", "Mar"]);
        
        let bounds = bar_chart.data_bounds().unwrap();
        assert_eq!(bounds.0, 90.0);  // min
        assert_eq!(bounds.1, 120.0); // max
    }

    #[test]
    fn test_bar_chart_with_negative_values() {
        let profit_data = [("Q1", -5.0), ("Q2", 10.0), ("Q3", -2.0), ("Q4", 15.0)];
        
        let bar_chart = BarPlot::new()
            .data(&profit_data)
            .auto_scale();
        
        let bounds = bar_chart.data_bounds().unwrap();
        assert_eq!(bounds.0, -5.0);
        assert_eq!(bounds.1, 15.0);
    }

    #[test]
    fn test_complete_bar_chart_workflow() {
        // 测试完整的柱状图工作流程
        let data = [("A", 10.0), ("B", 15.0), ("C", 8.0)];
        
        let bar_chart = BarPlot::new()
            .data(&data)
            .fill_color(Color::rgb(0.8, 0.4, 0.2))
            .stroke(Color::rgb(0.6, 0.2, 0.1), 2.0)
            .bar_width(0.8)
            .auto_scale();
        
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let y_scale = LinearScale::new(0.0, 20.0);
        
        let scene = Scene::new(plot_area)
            .add_y_axis(y_scale, Some("Value".to_string()))
            .add_bar_plot(bar_chart);
        
        let figure = Figure::new(600.0, 500.0)
            .add_scene(scene);
        
        let primitives = figure.generate_primitives();
        
        // 验证生成了预期的图元
        assert!(!primitives.is_empty());
        assert!(primitives.len() > 10); // 应该包含柱子、标签、轴线等
    }
}
