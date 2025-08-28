//! 面积图演示程序
//!
//! 展示 Vizuara 的面积图功能，包括单系列和堆叠面积图

use vizuara_core::{Color, LinearScale};
use vizuara_plots::{AreaChart, AreaFillMode, AreaSeries, PlotArea};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 面积图演示启动");
    println!("🎨 创建多种面积图样式...");

    // 1. 创建单系列面积图数据
    let temperature_data = vec![
        (1.0, 22.0), (2.0, 25.0), (3.0, 28.0), (4.0, 32.0),
        (5.0, 35.0), (6.0, 38.0), (7.0, 42.0), (8.0, 39.0),
        (9.0, 35.0), (10.0, 30.0), (11.0, 26.0), (12.0, 23.0),
    ];

    println!("🌡️  创建了 {} 个月的温度数据", temperature_data.len());

    // 2. 创建基础面积图
    let temp_chart = AreaChart::new()
        .single_series("月平均温度", &temperature_data)
        .auto_scale()
        .show_points(true, 3.0);

    // 3. 创建堆叠面积图数据
    let product_a_data = vec![
        (1.0, 20.0), (2.0, 25.0), (3.0, 30.0), (4.0, 28.0),
        (5.0, 35.0), (6.0, 40.0), (7.0, 38.0), (8.0, 42.0),
        (9.0, 45.0), (10.0, 48.0), (11.0, 50.0), (12.0, 52.0),
    ];
    
    let product_b_data = vec![
        (1.0, 15.0), (2.0, 18.0), (3.0, 22.0), (4.0, 25.0),
        (5.0, 28.0), (6.0, 30.0), (7.0, 32.0), (8.0, 35.0),
        (9.0, 38.0), (10.0, 40.0), (11.0, 42.0), (12.0, 45.0),
    ];
    
    let product_c_data = vec![
        (1.0, 10.0), (2.0, 12.0), (3.0, 15.0), (4.0, 18.0),
        (5.0, 20.0), (6.0, 22.0), (7.0, 25.0), (8.0, 28.0),
        (9.0, 30.0), (10.0, 32.0), (11.0, 35.0), (12.0, 38.0),
    ];

    println!("📦 创建了3个产品系列的销售数据");

    // 4. 创建堆叠面积图
    let product_series_a = AreaSeries::new("产品 A")
        .data(&product_a_data)
        .fill_color(Color::rgba(0.2, 0.6, 0.9, 0.7))
        .line_color(Color::rgb(0.1, 0.4, 0.7))
        .line_width(2.0);

    let product_series_b = AreaSeries::new("产品 B")
        .data(&product_b_data)
        .fill_color(Color::rgba(0.9, 0.5, 0.2, 0.7))
        .line_color(Color::rgb(0.7, 0.3, 0.1))
        .line_width(2.0);

    let product_series_c = AreaSeries::new("产品 C")
        .data(&product_c_data)
        .fill_color(Color::rgba(0.4, 0.8, 0.4, 0.7))
        .line_color(Color::rgb(0.2, 0.6, 0.2))
        .line_width(2.0);

    let stacked_chart = AreaChart::new()
        .add_series(product_series_a)
        .add_series(product_series_b)
        .add_series(product_series_c)
        .stacked()
        .auto_scale();

    println!("📊 创建堆叠面积图，包含 {} 个系列", stacked_chart.series_count());

    // 5. 创建流量数据（另一个单系列面积图）
    let traffic_data = vec![
        (0.0, 1200.0), (4.0, 800.0), (8.0, 600.0), (12.0, 2800.0),
        (16.0, 4200.0), (20.0, 3600.0), (24.0, 1800.0),
    ];

    let traffic_chart = AreaChart::new()
        .single_series("每日流量", &traffic_data)
        .fill_mode(AreaFillMode::ToZero)
        .smooth(true, 0.3)
        .auto_scale();

    println!("🚦 创建网站流量面积图");

    // 6. 创建场景布局
    println!("🎬 创建场景和坐标轴...");

    // 温度图场景
    let temp_area = PlotArea::new(80.0, 60.0, 350.0, 200.0);
    let temp_scene = Scene::new(temp_area)
        .title("月平均温度变化")
        .add_area_chart(temp_chart)
        .add_x_axis(LinearScale::new(0.0, 13.0), Some("月份".to_string()))
        .add_y_axis(LinearScale::new(20.0, 45.0), Some("温度(°C)".to_string()));

    // 堆叠产品销售图场景
    let product_area = PlotArea::new(480.0, 60.0, 350.0, 200.0);
    let product_scene = Scene::new(product_area)
        .title("产品销售堆叠图")
        .add_area_chart(stacked_chart)
        .add_x_axis(LinearScale::new(0.0, 13.0), Some("月份".to_string()))
        .add_y_axis(LinearScale::new(0.0, 140.0), Some("销售额(万)".to_string()));

    // 流量图场景
    let traffic_area = PlotArea::new(280.0, 320.0, 350.0, 200.0);
    let traffic_scene = Scene::new(traffic_area)
        .title("24小时网站流量")
        .add_area_chart(traffic_chart)
        .add_x_axis(LinearScale::new(0.0, 24.0), Some("小时".to_string()))
        .add_y_axis(LinearScale::new(0.0, 5000.0), Some("访问量".to_string()));

    // 7. 创建图形并显示
    let figure = Figure::new(900.0, 600.0)
        .title("Vizuara 面积图演示")
        .add_scene(temp_scene)
        .add_scene(product_scene)
        .add_scene(traffic_scene);

    println!("✨ 创建了包含 {} 个场景的图形", 3);

    // 8. 显示图形
    println!("🖥️  显示图形窗口...");
    show_figure(figure)?;

    println!("🎉 面积图演示完成！");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizuara_plots::{AreaDataPoint, AreaStyle};

    #[test]
    fn test_area_chart_creation() {
        let chart = AreaChart::new();
        assert_eq!(chart.series_count(), 0);
    }

    #[test]
    fn test_single_series_area() {
        let data = [(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
        let chart = AreaChart::new().single_series("测试", &data);
        
        assert_eq!(chart.series_count(), 1);
    }

    #[test]
    fn test_stacked_area_mode() {
        let chart = AreaChart::new().stacked();
        assert_eq!(chart.style.fill_mode, AreaFillMode::Stacked);
    }

    #[test]
    fn test_area_data_point_conversion() {
        let point_tuple = (1.5, 25.0);
        let point: AreaDataPoint = point_tuple.into();
        
        assert_eq!(point.x, 1.5);
        assert_eq!(point.y, 25.0);
    }

    #[test]
    fn test_area_series_builder() {
        let data = [(0.0, 5.0), (1.0, 10.0)];
        let series = AreaSeries::new("测试系列")
            .data(&data)
            .fill_color(Color::rgb(1.0, 0.0, 0.0))
            .line_width(3.0);
        
        assert_eq!(series.label, "测试系列");
        assert_eq!(series.data.len(), 2);
        assert_eq!(series.line_width, 3.0);
    }

    #[test]
    fn test_multiple_series_chart() {
        let series1 = AreaSeries::new("系列1").data(&[(0.0, 10.0), (1.0, 15.0)]);
        let series2 = AreaSeries::new("系列2").data(&[(0.0, 5.0), (1.0, 8.0)]);
        
        let chart = AreaChart::new()
            .add_series(series1)
            .add_series(series2);
        
        assert_eq!(chart.series_count(), 2);
    }
}
