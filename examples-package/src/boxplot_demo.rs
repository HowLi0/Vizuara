use vizuara_plots::{BoxPlot, BoxPlotStyle, PlotArea};
use vizuara_scene::{Scene, Figure};
use vizuara_core::{Color, LinearScale};
use vizuara_window::show_figure;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Vizuara 箱线图演示启动！");

    // 创建示例数据 - 模拟不同组的测试成绩
    let group_a_scores = vec![
        85.0, 87.0, 89.0, 91.0, 88.0, 86.0, 90.0, 84.0, 92.0, 88.0,
        85.0, 89.0, 87.0, 90.0, 86.0, 83.0, 91.0, 88.0, 89.0, 87.0
    ];
    
    let group_b_scores = vec![
        78.0, 82.0, 85.0, 79.0, 83.0, 81.0, 84.0, 77.0, 86.0, 80.0,
        79.0, 83.0, 82.0, 85.0, 81.0, 76.0, 84.0, 82.0, 80.0, 78.0
    ];
    
    let group_c_scores = vec![
        92.0, 95.0, 88.0, 94.0, 96.0, 89.0, 93.0, 91.0, 97.0, 90.0,
        94.0, 88.0, 92.0, 95.0, 89.0, 96.0, 93.0, 91.0, 94.0, 87.0
    ];

    // 添加一些异常值数据
    let group_d_scores = vec![
        75.0, 77.0, 78.0, 76.0, 79.0, 74.0, 80.0, 73.0, 81.0, 77.0,
        65.0, // 异常值 - 偏低
        95.0, // 异常值 - 偏高
        76.0, 78.0, 77.0, 79.0, 75.0, 80.0, 74.0, 76.0
    ];

    println!("📈 生成箱线图数据...");
    
    // 创建数据组
    let data_groups = &[
        ("班级A", group_a_scores),
        ("班级B", group_b_scores), 
        ("班级C", group_c_scores),
        ("班级D", group_d_scores),
    ];

    // 创建自定义样式的箱线图
    let custom_style = BoxPlotStyle {
        box_fill_color: Color::new(0.3, 0.7, 1.0, 0.7),  // 半透明蓝色
        box_stroke_color: Color::rgb(0.1, 0.4, 0.8),      // 深蓝色边框
        box_stroke_width: 2.5,
        median_color: Color::rgb(1.0, 0.2, 0.2),          // 红色中位数线
        median_width: 3.0,
        whisker_color: Color::rgb(0.2, 0.2, 0.2),         // 深灰色须线
        whisker_width: 2.0,
        outlier_color: Color::rgb(1.0, 0.5, 0.0),         // 橙色异常值
        outlier_size: 5.0,
        box_width: 0.7,
    };

    let boxplot = BoxPlot::new()
        .from_data_groups(data_groups)
        .style(custom_style)
        .auto_range();

    println!("📊 创建场景和坐标轴...");

    // 创建绘图区域
    let plot_area = PlotArea::new(80.0, 60.0, 640.0, 480.0);

    // 创建坐标轴
    let x_scale = LinearScale::new(0.0, 4.0);  // 4个组
    let y_scale = LinearScale::new(60.0, 100.0);  // 成绩范围

    // 创建场景
    let scene = Scene::new(plot_area)
        .add_x_axis(x_scale, Some("班级".to_string()))
        .add_y_axis(y_scale, Some("考试成绩".to_string()))
        .add_boxplot(boxplot)
        .title("各班级考试成绩箱线图");

    println!("🖼️  生成图形...");
    
    // 创建图形对象
    let figure = Figure::new(800.0, 600.0)
        .title("Vizuara 箱线图演示")
        .add_scene(scene);
    
    println!("✅ 图形创建完成");
    
    // 显示图形
    show_figure(figure)?;
    
    // 等待一段时间以便观察
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    println!("🎯 箱线图演示完成！");
    println!("💡 图表说明:");
    println!("   • 箱子显示 Q1-Q3 四分位距");
    println!("   • 红色线表示中位数");
    println!("   • 须线显示数据范围(不含异常值)");
    println!("   • 橙色圆点表示异常值");
    
    Ok(())
}
