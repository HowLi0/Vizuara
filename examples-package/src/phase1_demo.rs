//! 阶段1示例：完整的散点图展示
//! 
//! 这个示例展示了 Vizuara 阶段1的核心功能：
//! - 散点图渲染
//! - 坐标轴系统
//! - 高级 API 使用

use vizuara_core::{LinearScale, Color};
use vizuara_plots::{ScatterPlot, PlotArea};
use vizuara_scene::{Scene, Figure};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Vizuara 阶段1 功能展示");
    
    // 1. 创建测试数据
    let data = vec![
        (1.0, 2.1), (1.5, 2.8), (2.0, 3.2), (2.5, 2.9),
        (3.0, 4.1), (3.5, 4.8), (4.0, 5.2), (4.5, 4.9),
        (5.0, 6.1), (5.5, 6.8), (6.0, 7.2), (6.5, 6.9),
    ];
    
    println!("📊 创建散点图，包含 {} 个数据点", data.len());
    
    // 2. 创建散点图
    let scatter = ScatterPlot::new()
        .data(&data)
        .color(Color::rgb(0.8, 0.2, 0.4))  // 粉红色
        .size(8.0)
        .auto_scale();
        
    // 3. 设置坐标轴比例尺
    let x_scale = LinearScale::new(0.0, 7.0);
    let y_scale = LinearScale::new(0.0, 8.0);
    
    // 4. 创建绘图区域和场景
    let plot_area = PlotArea::new(100.0, 100.0, 600.0, 400.0);
    let scene = Scene::new(plot_area)
        .add_x_axis(x_scale, Some("X 轴 (输入值)".to_string()))
        .add_y_axis(y_scale, Some("Y 轴 (输出值)".to_string()))
        .add_scatter_plot(scatter)
        .title("散点图示例");
    
    // 5. 创建图形对象
    let figure = Figure::new(800.0, 600.0)
        .title("Vizuara 阶段1 - 科学可视化展示")
        .add_scene(scene);
    
    // 6. 生成渲染图元
    let primitives = figure.generate_primitives();
    println!("✅ 成功生成 {} 个渲染图元", primitives.len());
    
    // 7. 打印图元统计
    let mut point_count = 0;
    let mut line_count = 0;
    let mut text_count = 0;
    let mut rect_count = 0;
    
    for primitive in &primitives {
        match primitive {
            vizuara_core::Primitive::Points(points) => {
                point_count += points.len();
            }
            vizuara_core::Primitive::Line { .. } => {
                line_count += 1;
            }
            vizuara_core::Primitive::Text { .. } => {
                text_count += 1;
            }
            vizuara_core::Primitive::Rectangle { .. } => {
                rect_count += 1;
            }
            _ => {}
        }
    }
    
    println!("📈 图元统计:");
    println!("  - 数据点: {} 个", point_count);
    println!("  - 线条: {} 条", line_count);
    println!("  - 文本: {} 个", text_count);
    println!("  - 矩形: {} 个", rect_count);
    
    println!("🎉 阶段1 核心功能验证完成！");
    println!();
    println!("已实现的功能:");
    println!("✅ 核心数据结构 (Primitive, Style, Scale)");
    println!("✅ 坐标轴组件 (Axis)");
    println!("✅ 散点图 (ScatterPlot)");
    println!("✅ 场景管理 (Scene, Figure)");
    println!("✅ 高级 API 设计");
    println!();
    println!("下一步计划:");
    println!("🔄 完善渲染器集成");
    println!("🎨 增加更多图表类型");
    println!("🖱️  添加交互功能");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase1_complete_workflow() {
        // 测试完整的阶段1工作流程
        let data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 1.0)];
        
        let scatter = ScatterPlot::new()
            .data(&data)
            .color(Color::rgb(1.0, 0.0, 0.0))
            .auto_scale();
        
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let x_scale = LinearScale::new(0.0, 4.0);
        let y_scale = LinearScale::new(0.0, 4.0);
        
        let scene = Scene::new(plot_area)
            .add_x_axis(x_scale, Some("X".to_string()))
            .add_y_axis(y_scale, Some("Y".to_string()))
            .add_scatter_plot(scatter);
        
        let figure = Figure::new(600.0, 500.0)
            .add_scene(scene);
        
        let primitives = figure.generate_primitives();
        
        // 验证生成了预期的图元
        assert!(!primitives.is_empty());
        assert!(primitives.len() > 10); // 应该包含轴线、刻度、标签、数据点等
    }
}
