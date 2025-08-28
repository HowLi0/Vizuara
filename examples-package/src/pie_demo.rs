//! 饼图演示程序
//!
//! 展示 Vizuara 的饼图和圆环图功能

use vizuara_core::Color;
use vizuara_plots::{PieChart, PlotArea};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🥧 饼图演示启动");
    println!("🎨 创建多种饼图样式...");

    // 1. 创建销售数据
    let sales_data = [
        ("移动端", 45.2),
        ("桌面端", 32.8),
        ("平板端", 15.3),
        ("其他", 6.7),
    ];

    println!("✅ 创建了 {} 种渠道的销售数据", sales_data.len());

    // 2. 创建基础饼图
    let pie_chart = PieChart::new()
        .data(&sales_data)
        .center(200.0, 200.0)
        .radius(120.0)
        .title("销售渠道分布")
        .show_percentage(true);

    println!("📊 创建基础饼图: 半径{}px", 120.0);

    // 3. 创建圆环图
    let donut_chart = PieChart::new()
        .data(&sales_data)
        .center(500.0, 200.0)
        .donut(60.0, 120.0)
        .gap_angle(0.05)
        .title("销售渠道分布（圆环图）")
        .stroke(Color::rgb(1.0, 1.0, 1.0), 2.0);

    println!("🍩 创建圆环图: 内半径{}px，外半径{}px", 60.0, 120.0);

    // 4. 创建预算分配数据
    let budget_data = [
        ("研发", 40.0),
        ("市场营销", 25.0),
        ("运营", 20.0),
        ("人力资源", 10.0),
        ("其他", 5.0),
    ];

    let budget_chart = PieChart::new()
        .data(&budget_data)
        .center(200.0, 450.0)
        .radius(100.0)
        .start_angle(0.0) // 从右侧开始
        .gap_angle(0.02)
        .title("预算分配")
        .labels(true, 11.0, Color::rgb(0.1, 0.1, 0.1), 1.3);

    println!("💰 创建预算分配饼图: {} 个分类", budget_data.len());

    // 5. 创建自定义颜色的圆环图
    let custom_donut = PieChart::new()
        .labels_values(&["优秀", "良好", "一般", "较差"], &[35.0, 40.0, 20.0, 5.0])
        .center(500.0, 450.0)
        .donut(50.0, 110.0)
        .stroke(Color::rgb(0.8, 0.8, 0.8), 1.5)
        .title("满意度调查")
        .show_percentage(false);

    println!("😊 创建满意度调查圆环图");

    // 6. 创建场景并添加所有图表
    println!("🎬 创建场景和图形...");

    // 创建绘图区域（覆盖整个画布）
    let plot_area = PlotArea::new(0.0, 0.0, 800.0, 700.0);
    
    // 创建场景并添加所有饼图
    let scene = Scene::new(plot_area)
        .add_pie_chart(pie_chart)
        .add_pie_chart(donut_chart)
        .add_pie_chart(budget_chart)
        .add_pie_chart(custom_donut);

    let primitives = scene.generate_primitives();
    println!("✨ 生成了 {} 个渲染图元", primitives.len());

    let figure = Figure::new(800.0, 700.0)
        .title("Vizuara 饼图与圆环图演示")
        .add_scene(scene);

    // 7. 显示图形
    println!("🖥️  显示图形窗口...");
    show_figure(figure)?;

    println!("🎉 饼图演示完成！");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizuara_plots::PieData;

    #[test]
    fn test_pie_chart_creation() {
        let data = [("A", 30.0), ("B", 70.0)];
        let chart = PieChart::new().data(&data);
        
        assert_eq!(chart.data_len(), 2);
        assert_eq!(chart.total_value(), 100.0);
    }

    #[test]
    fn test_donut_chart_setup() {
        let chart = PieChart::new()
            .donut(30.0, 80.0)
            .gap_angle(0.1);
        
        assert_eq!(chart.style.inner_radius, 30.0);
        assert_eq!(chart.style.outer_radius, 80.0);
    }

    #[test]
    fn test_pie_data_conversion() {
        let data_tuple = ("测试", 50.0);
        let pie_data: PieData = data_tuple.into();
        
        assert_eq!(pie_data.label, "测试");
        assert_eq!(pie_data.value, 50.0);
    }

    #[test]
    fn test_pie_primitives_generation() {
        let data = [("A", 50.0), ("B", 50.0)];
        let chart = PieChart::new()
            .data(&data)
            .center(100.0, 100.0)
            .radius(50.0);
        
        let primitives = chart.generate_primitives(PlotArea::new(0.0, 0.0, 200.0, 200.0));
        assert!(!primitives.is_empty());
    }
}
