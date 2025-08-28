//! 雷达图演示程序
//!
//! 展示 Vizuara 的雷达图功能，用于多维数据可视化

use vizuara_core::Color;
use vizuara_plots::{PlotArea, RadarChart, RadarDimension, RadarSeries};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🕸️ 雷达图演示启动");
    println!("🎨 创建多维数据可视化...");

    // 1. 创建能力评估雷达图
    let skill_dimensions = vec![
        RadarDimension::new("编程能力", 0.0, 10.0),
        RadarDimension::new("算法思维", 0.0, 10.0),
        RadarDimension::new("系统设计", 0.0, 10.0),
        RadarDimension::new("团队协作", 0.0, 10.0),
        RadarDimension::new("沟通表达", 0.0, 10.0),
        RadarDimension::new("学习能力", 0.0, 10.0),
    ];

    let alice_skills = RadarSeries::new("Alice", vec![8.5, 9.0, 7.5, 8.0, 7.0, 9.5])
        .fill_color(Color::rgba(0.2, 0.6, 0.9, 0.3))
        .line_color(Color::rgb(0.2, 0.6, 0.9))
        .line_width(2.5);

    let bob_skills = RadarSeries::new("Bob", vec![7.0, 8.5, 9.0, 6.5, 8.5, 7.5])
        .fill_color(Color::rgba(0.9, 0.5, 0.2, 0.3))
        .line_color(Color::rgb(0.9, 0.5, 0.2))
        .line_width(2.5);

    let skill_radar = RadarChart::new()
        .dimensions(skill_dimensions)
        .add_series(alice_skills)
        .add_series(bob_skills)
        .center_radius(200.0, 200.0, 120.0)
        .title("团队成员技能对比")
        .grid_style(Color::rgb(0.7, 0.7, 0.7), 1.0, 5);

    println!("👥 创建了 {} 维度的技能对比雷达图", skill_radar.dimension_count());

    // 2. 创建产品性能雷达图
    let performance_chart = RadarChart::new()
        .simple_dimensions(&["性能", "可用性", "安全性", "可扩展性", "用户体验", "成本效益"], 0.0, 100.0)
        .add_data("当前版本", vec![75.0, 85.0, 90.0, 70.0, 80.0, 85.0])
        .add_data("目标版本", vec![90.0, 95.0, 95.0, 85.0, 90.0, 80.0])
        .center_radius(550.0, 200.0, 120.0)
        .title("产品性能评估");

    println!("📊 创建了 {} 系列的性能评估雷达图", performance_chart.series_count());

    // 3. 创建城市生活质量雷达图
    let city_radar = RadarChart::new()
        .simple_dimensions(&["交通便利", "环境质量", "教育资源", "医疗服务", "文化娱乐", "生活成本"], 0.0, 10.0)
        .add_data("北京", vec![8.0, 6.0, 9.0, 8.5, 9.5, 4.0])
        .add_data("上海", vec![8.5, 6.5, 8.5, 8.0, 9.0, 3.5])
        .add_data("深圳", vec![8.0, 7.0, 8.0, 7.5, 8.0, 5.0])
        .center_radius(200.0, 500.0, 120.0)
        .title("主要城市生活质量对比");

    println!("🏙️ 创建了 {} 个城市的生活质量对比", city_radar.series_count());

    // 4. 创建兴趣爱好雷达图
    let interest_radar = RadarChart::new()
        .simple_dimensions(&["运动", "音乐", "阅读", "旅行", "美食", "游戏"], 1.0, 5.0)
        .add_data("张三", vec![4.0, 3.0, 5.0, 4.5, 3.5, 2.0])
        .center_radius(550.0, 500.0, 120.0)
        .title("个人兴趣分布");

    println!("🎯 创建了兴趣爱好分布雷达图");

    // 5. 创建场景并添加所有雷达图
    println!("🎬 创建场景和图形...");

    // 创建绘图区域（覆盖整个画布）
    let plot_area = PlotArea::new(0.0, 0.0, 900.0, 700.0);
    
    // 创建场景并添加所有雷达图
    let scene = Scene::new(plot_area)
        .add_radar_chart(skill_radar)
        .add_radar_chart(performance_chart)
        .add_radar_chart(city_radar)
        .add_radar_chart(interest_radar);

    let primitives = scene.generate_primitives();
    println!("✨ 生成了 {} 个渲染图元", primitives.len());

    let figure = Figure::new(900.0, 700.0)
        .title("Vizuara 雷达图演示")
        .add_scene(scene);

    // 6. 显示图形
    println!("🖥️ 显示图形窗口...");
    show_figure(figure)?;

    println!("🎉 雷达图演示完成！");
    Ok(())
}
