use vizuara_plots::{Heatmap, ColorMap, PlotArea};
use vizuara_scene::{Scene, Figure};
use vizuara_window::show_figure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Vizuara 热力图演示启动！");
    
    // 1. 创建示例数据 - 模拟产品销售热力图
    let sales_data = vec![
        vec![23.5, 45.2, 67.8, 34.1, 56.9, 78.3, 45.6],  // 产品A
        vec![34.7, 56.1, 23.4, 67.9, 45.2, 34.8, 89.1],  // 产品B  
        vec![45.9, 78.3, 56.7, 23.1, 67.4, 56.8, 34.2],  // 产品C
        vec![67.2, 34.5, 89.1, 45.6, 23.8, 78.9, 56.3],  // 产品D
        vec![78.8, 23.9, 45.3, 78.2, 89.7, 45.1, 67.5],  // 产品E
    ];

    let weeks = &["第1周", "第2周", "第3周", "第4周", "第5周", "第6周", "第7周"];
    let products = &["产品A", "产品B", "产品C", "产品D", "产品E"];

    println!("📊 创建 {}x{} 销售数据热力图", sales_data.len(), sales_data[0].len());

    // 2. 创建多种风格的热力图
    
    // 经典蓝-白-红热力图
    let heatmap_classic = Heatmap::new()
        .data(&sales_data)
        .x_labels(weeks)
        .y_labels(products)
        .color_map(ColorMap::BlueWhiteRed)
        .show_grid(true)
        .show_values(true)
        .auto_range();

    println!("🎨 经典蓝-白-红配色方案");

    // 3. 创建场景
    let plot_area = PlotArea::new(80.0, 60.0, 600.0, 400.0);
    
    let scene = Scene::new(plot_area)
        .title("产品销售热力图 - 周销售额 (千元)")
        .add_heatmap(heatmap_classic);

    // 4. 创建图形并显示
    let figure = Figure::new(800.0, 600.0)
        .title("Vizuara 热力图演示")
        .add_scene(scene);

    println!("🚀 启动窗口渲染...");
    println!("💡 提示: ESC 退出, R 刷新");
    println!("🔥 热力图展示不同产品在各周的销售表现");

    show_figure(figure)?;

    println!("✅ 演示完成!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_demo_data_creation() {
        let sales_data = vec![
            vec![23.5, 45.2, 67.8],
            vec![34.7, 56.1, 23.4],
        ];

        let heatmap = Heatmap::new()
            .data(&sales_data)
            .x_labels(&["Week 1", "Week 2", "Week 3"])
            .y_labels(&["Product A", "Product B"])
            .auto_range();

        assert_eq!(heatmap.dimensions(), (2, 3));
        assert_eq!(heatmap.get_value(0, 1), Some(45.2));
    }

    #[test]
    fn test_color_map_variations() {
        let test_data = vec![vec![0.0, 0.5, 1.0]];
        
        // 测试不同颜色映射
        let maps = vec![
            ColorMap::BlueWhiteRed,
            ColorMap::BlueGreen,
            ColorMap::Grayscale,
            ColorMap::Rainbow,
        ];

        for color_map in maps {
            let heatmap = Heatmap::new()
                .data(&test_data)
                .color_map(color_map)
                .auto_range();
            
            let plot_area = PlotArea::new(0.0, 0.0, 100.0, 50.0);
            let primitives = heatmap.generate_primitives(plot_area);
            
            // 应该生成矩形图元
            assert!(!primitives.is_empty());
        }
    }

    #[test]
    fn test_heatmap_with_custom_range() {
        let data = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        
        let heatmap = Heatmap::new()
            .data(&data)
            .value_range(0.0, 50.0); // 自定义范围
        
        let plot_area = PlotArea::new(0.0, 0.0, 100.0, 100.0);
        let primitives = heatmap.generate_primitives(plot_area);
        
        assert!(primitives.len() >= 4); // 至少4个单元格
    }
}
