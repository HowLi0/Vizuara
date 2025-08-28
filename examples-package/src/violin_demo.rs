//! 小提琴图演示程序
//!
//! 展示 Vizuara 的小提琴图功能，用于显示数据分布

use vizuara_core::{Color, LinearScale};
use vizuara_plots::{PlotArea, ViolinPlot, ViolinStyle};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn generate_normal_data(mean: f32, std_dev: f32, size: usize) -> Vec<f32> {
    use rand::thread_rng;
    use rand::Rng;
    
    let mut rng = thread_rng();
    let mut data = Vec::new();
    
    for _ in 0..size {
        // 使用 Box-Muller 变换生成正态分布
        let u1 = rng.gen::<f32>();
        let u2 = rng.gen::<f32>();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
        data.push(mean + std_dev * z0);
    }
    
    data
}

fn generate_bimodal_data(mean1: f32, mean2: f32, std_dev: f32, size: usize) -> Vec<f32> {
    use rand::thread_rng;
    use rand::Rng;
    
    let mut rng = thread_rng();
    let mut data = Vec::new();
    
    for _ in 0..size {
        if rng.gen::<f32>() < 0.5 {
            data.extend(generate_normal_data(mean1, std_dev, 1));
        } else {
            data.extend(generate_normal_data(mean2, std_dev, 1));
        }
    }
    
    data
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎻 小提琴图演示启动");
    println!("🎨 创建多种数据分布可视化...");

    // 1. 创建不同分布的测试数据
    let group_a_data = generate_normal_data(75.0, 10.0, 200);
    let group_b_data = generate_normal_data(82.0, 8.0, 180);
    let group_c_data = generate_bimodal_data(70.0, 90.0, 5.0, 220);
    let group_d_data = generate_normal_data(78.0, 15.0, 160);

    println!("📊 生成了4组不同分布的数据");
    println!("   - 组A: 正态分布 (μ=75, σ=10, n=200)");
    println!("   - 组B: 正态分布 (μ=82, σ=8, n=180)");
    println!("   - 组C: 双峰分布 (μ₁=70, μ₂=90, σ=5, n=220)");
    println!("   - 组D: 正态分布 (μ=78, σ=15, n=160)");

    // 2. 创建基础小提琴图
    let data_groups = [
        ("组A", group_a_data),
        ("组B", group_b_data),
        ("组C", group_c_data),
        ("组D", group_d_data),
    ];

    let basic_violin = ViolinPlot::new()
        .from_data_groups(&data_groups)
        .auto_range()
        .title("基础小提琴图");

    println!("🎼 创建基础小提琴图，包含 {} 个组", basic_violin.group_count());

    // 3. 创建自定义样式的小提琴图
    let custom_style = ViolinStyle {
        violin_fill_color: Color::rgba(0.4, 0.7, 0.9, 0.7),
        violin_stroke_color: Color::rgb(0.2, 0.5, 0.7),
        violin_stroke_width: 2.0,
        median_color: Color::rgb(1.0, 1.0, 1.0),
        median_width: 3.0,
        mean_color: Color::rgb(1.0, 0.2, 0.2),
        mean_size: 5.0,
        quartile_color: Color::rgb(0.0, 0.0, 0.0),
        quartile_width: 2.0,
        outlier_color: Color::rgb(1.0, 0.5, 0.0),
        outlier_size: 4.0,
        violin_width: 0.7,
        show_box: true,
        box_width: 0.15,
        box_color: Color::rgb(0.2, 0.2, 0.2),
        show_points: false,
        point_color: Color::rgba(0.1, 0.1, 0.1, 0.3),
        point_size: 1.5,
        point_alpha: 0.3,
    };

    let styled_violin = ViolinPlot::new()
        .from_data_groups(&data_groups)
        .style(custom_style)
        .auto_range()
        .title("自定义样式小提琴图");

    println!("🎨 创建自定义样式小提琴图");

    // 4. 创建显示数据点的小提琴图
    let exam_scores = [
        ("数学", vec![85.0, 92.0, 78.0, 95.0, 88.0, 82.0, 91.0, 87.0, 89.0, 84.0, 
                     93.0, 86.0, 90.0, 81.0, 94.0, 83.0, 88.0, 92.0, 85.0, 89.0]),
        ("英语", vec![79.0, 85.0, 88.0, 82.0, 91.0, 86.0, 84.0, 89.0, 87.0, 83.0,
                     90.0, 85.0, 88.0, 81.0, 86.0, 84.0, 89.0, 87.0, 82.0, 88.0]),
        ("物理", vec![72.0, 89.0, 85.0, 78.0, 92.0, 86.0, 81.0, 88.0, 84.0, 79.0,
                     91.0, 87.0, 83.0, 76.0, 89.0, 85.0, 82.0, 90.0, 86.0, 84.0]),
    ];

    let exam_violin = ViolinPlot::new()
        .from_data_groups(&exam_scores)
        .show_points(true, 2.0, 0.4)
        .violin_color(Color::rgba(0.6, 0.8, 0.4, 0.6), Color::rgb(0.4, 0.6, 0.2))
        .auto_range()
        .title("考试成绩分布（含数据点）");

    println!("📝 创建考试成绩小提琴图，显示数据点");

    // 5. 创建场景布局
    println!("🎬 创建场景和坐标轴...");

    // 基础小提琴图场景
    let basic_area = PlotArea::new(80.0, 60.0, 350.0, 220.0);
    let basic_scene = Scene::new(basic_area)
        .title("数据分布对比")
        .add_violin_plot(basic_violin)
        .add_y_axis(LinearScale::new(40.0, 120.0), Some("数值".to_string()));

    // 自定义样式小提琴图场景
    let styled_area = PlotArea::new(480.0, 60.0, 350.0, 220.0);
    let styled_scene = Scene::new(styled_area)
        .title("样式定制效果")
        .add_violin_plot(styled_violin)
        .add_y_axis(LinearScale::new(40.0, 120.0), Some("数值".to_string()));

    // 考试成绩小提琴图场景
    let exam_area = PlotArea::new(280.0, 340.0, 350.0, 200.0);
    let exam_scene = Scene::new(exam_area)
        .title("学科成绩分析")
        .add_violin_plot(exam_violin)
        .add_y_axis(LinearScale::new(70.0, 100.0), Some("分数".to_string()));

    // 6. 创建图形并显示
    let figure = Figure::new(900.0, 600.0)
        .title("Vizuara 小提琴图演示")
        .add_scene(basic_scene)
        .add_scene(styled_scene)
        .add_scene(exam_scene);

    println!("✨ 创建了包含 {} 个场景的图形", 3);

    // 7. 显示图形
    println!("🖥️  显示图形窗口...");
    show_figure(figure)?;

    println!("🎉 小提琴图演示完成！");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizuara_plots::{DensityEstimate, ViolinStatistics};

    #[test]
    fn test_violin_plot_creation() {
        let plot = ViolinPlot::new();
        assert_eq!(plot.group_count(), 0);
    }

    #[test]
    fn test_violin_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = ViolinStatistics::from_data(data);
        
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_density_estimation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let density = DensityEstimate::from_data(&data, Some(1.0));
        
        assert!(!density.points.is_empty());
        assert!(!density.densities.is_empty());
        assert!(density.max_density > 0.0);
    }

    #[test]
    fn test_violin_from_data_groups() {
        let data_groups = [
            ("组A", vec![1.0, 2.0, 3.0]),
            ("组B", vec![4.0, 5.0, 6.0]),
        ];
        
        let plot = ViolinPlot::new().from_data_groups(&data_groups);
        assert_eq!(plot.group_count(), 2);
    }

    #[test]
    fn test_violin_group_creation() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let group = ViolinGroup::from_data("测试组", data);
        
        assert_eq!(group.label, "测试组");
        assert_eq!(group.statistics.median, 30.0);
    }

    #[test]
    fn test_violin_style_customization() {
        let plot = ViolinPlot::new()
            .violin_color(Color::rgb(1.0, 0.0, 0.0), Color::rgb(0.5, 0.0, 0.0))
            .show_box(true, 0.2)
            .show_points(true, 3.0, 0.5);
        
        assert_eq!(plot.style.violin_fill_color, Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(plot.style.violin_stroke_color, Color::rgb(0.5, 0.0, 0.0));
        assert!(plot.style.show_box);
        assert_eq!(plot.style.box_width, 0.2);
        assert!(plot.style.show_points);
        assert_eq!(plot.style.point_size, 3.0);
        assert_eq!(plot.style.point_alpha, 0.5);
    }
}
