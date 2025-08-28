//! 高级图表演示
//!
//! 演示5种新实现的高级可视化图表类型：
//! - 密度图 (DensityPlot)
//! - 等高线图 (ContourPlot) 
//! - 桑基图 (SankeyDiagram)
//! - 树状图 (Treemap)
//! - 平行坐标图 (ParallelCoordinates)

use vizuara_plots::*;
use vizuara_scene::Scene;
use vizuara_window::{Window, WindowManager};
use vizuara_core::Color;
use nalgebra::Point2;

fn main() {
    println!("🎨 Vizuara 高级图表演示");
    println!("展示5种新实现的高级可视化图表类型");

    let mut window_manager = WindowManager::new();
    
    // 创建密度图示例
    create_density_demo(&mut window_manager);
    
    // 创建等高线图示例
    create_contour_demo(&mut window_manager);
    
    // 创建桑基图示例
    create_sankey_demo(&mut window_manager);
    
    // 创建树状图示例
    create_treemap_demo(&mut window_manager);
    
    // 创建平行坐标图示例
    create_parallel_demo(&mut window_manager);

    println!("✅ 所有示例窗口已创建！");
}

/// 创建密度图演示
fn create_density_demo(window_manager: &mut WindowManager) {
    println!("📊 创建密度图演示...");
    
    let mut window = Window::new("密度图演示 - Kernel Density Estimation", 800, 600);
    let mut scene = Scene::new();

    // 生成示例数据 - 混合正态分布
    let mut data = Vec::new();
    for i in 0..100 {
        // 第一个峰值
        let value1 = 2.0 + (i as f32 / 100.0) * 0.5 + (rand::random::<f32>() - 0.5) * 1.0;
        data.push(value1);
        
        // 第二个峰值
        let value2 = 7.0 + (i as f32 / 100.0) * 0.3 + (rand::random::<f32>() - 0.5) * 0.8;
        data.push(value2);
    }

    // 创建密度图
    let mut density_plot = DensityPlot::new(data)
        .title("数据密度分布".to_string())
        .kernel(KernelType::Gaussian)
        .bandwidth(0.5)
        .resolution(200);
    
    density_plot.style_mut().fill_color = Color::rgba(0.2, 0.6, 0.8, 0.6);
    density_plot.style_mut().line_color = Color::rgb(0.1, 0.4, 0.7);
    density_plot.style_mut().show_points = true;

    let plot_area = PlotArea::new(50.0, 50.0, 700.0, 500.0);
    scene.add_density_plot(density_plot, plot_area);

    window.set_scene(scene);
    window_manager.add_window(window);
}

/// 创建等高线图演示
fn create_contour_demo(window_manager: &mut WindowManager) {
    println!("🗺️ 创建等高线图演示...");
    
    let mut window = Window::new("等高线图演示 - Contour Plot", 800, 600);
    let mut scene = Scene::new();

    // 生成3D数据 - 高斯山峰
    let width = 20;
    let height = 20;
    let mut data = vec![vec![0.0; width]; height];
    
    for i in 0..height {
        for j in 0..width {
            let x = (j as f32 - width as f32 / 2.0) / 3.0;
            let y = (i as f32 - height as f32 / 2.0) / 3.0;
            // 创建两个高斯峰
            let peak1 = (-((x - 1.0).powi(2) + (y - 1.0).powi(2))).exp() * 5.0;
            let peak2 = (-((x + 1.0).powi(2) + (y + 1.0).powi(2))).exp() * 3.0;
            data[i][j] = peak1 + peak2;
        }
    }

    // 创建等高线图
    let mut contour_plot = ContourPlot::new(data)
        .title("3D高度等高线".to_string())
        .auto_levels(8);
    
    contour_plot.style_mut().filled = false;
    contour_plot.style_mut().show_labels = true;

    let plot_area = PlotArea::new(50.0, 50.0, 700.0, 500.0);
    scene.add_contour_plot(contour_plot, plot_area);

    window.set_scene(scene);
    window_manager.add_window(window);
}

/// 创建桑基图演示
fn create_sankey_demo(window_manager: &mut WindowManager) {
    println!("🌊 创建桑基图演示...");
    
    let mut window = Window::new("桑基图演示 - Sankey Diagram", 900, 600);
    let mut scene = Scene::new();

    // 创建能源流动桑基图
    let mut sankey = SankeyDiagram::new()
        .title("能源流动图".to_string());

    // 添加节点
    sankey.add_node("煤炭", Color::rgb(0.4, 0.2, 0.1));
    sankey.add_node("石油", Color::rgb(0.2, 0.2, 0.2));
    sankey.add_node("天然气", Color::rgb(0.3, 0.4, 0.8));
    sankey.add_node("水力", Color::rgb(0.2, 0.6, 0.8));
    sankey.add_node("核能", Color::rgb(0.8, 0.8, 0.2));
    sankey.add_node("发电", Color::rgb(0.6, 0.6, 0.6));
    sankey.add_node("工业", Color::rgb(0.8, 0.4, 0.2));
    sankey.add_node("民用", Color::rgb(0.2, 0.8, 0.2));
    sankey.add_node("交通", Color::rgb(0.8, 0.2, 0.2));

    // 添加链接
    sankey.add_link("煤炭", "发电", 45.0, Color::rgba(0.4, 0.2, 0.1, 0.6));
    sankey.add_link("石油", "发电", 15.0, Color::rgba(0.2, 0.2, 0.2, 0.6));
    sankey.add_link("天然气", "发电", 25.0, Color::rgba(0.3, 0.4, 0.8, 0.6));
    sankey.add_link("水力", "发电", 10.0, Color::rgba(0.2, 0.6, 0.8, 0.6));
    sankey.add_link("核能", "发电", 5.0, Color::rgba(0.8, 0.8, 0.2, 0.6));
    
    sankey.add_link("发电", "工业", 40.0, Color::rgba(0.8, 0.4, 0.2, 0.6));
    sankey.add_link("发电", "民用", 35.0, Color::rgba(0.2, 0.8, 0.2, 0.6));
    sankey.add_link("发电", "交通", 25.0, Color::rgba(0.8, 0.2, 0.2, 0.6));

    sankey.style_mut().node_width = 20.0;
    sankey.style_mut().link_opacity = 0.7;

    let plot_area = PlotArea::new(50.0, 50.0, 800.0, 500.0);
    scene.add_sankey_diagram(sankey, plot_area);

    window.set_scene(scene);
    window_manager.add_window(window);
}

/// 创建树状图演示
fn create_treemap_demo(window_manager: &mut WindowManager) {
    println!("🌳 创建树状图演示...");
    
    let mut window = Window::new("树状图演示 - Treemap", 800, 600);
    let mut scene = Scene::new();

    // 创建软件公司架构树状图
    let mut treemap = Treemap::new()
        .title("软件公司部门结构".to_string())
        .color_scheme(ColorScheme::Category10);

    // 构建层次结构
    let mut root = TreemapItem::new("公司总部".to_string(), 0.0);
    
    // 技术部门
    let mut tech_dept = TreemapItem::new("技术部".to_string(), 0.0);
    tech_dept.add_child(TreemapItem::new("前端组".to_string(), 15.0));
    tech_dept.add_child(TreemapItem::new("后端组".to_string(), 25.0));
    tech_dept.add_child(TreemapItem::new("数据组".to_string(), 12.0));
    tech_dept.add_child(TreemapItem::new("DevOps".to_string(), 8.0));
    
    // 产品部门
    let mut product_dept = TreemapItem::new("产品部".to_string(), 0.0);
    product_dept.add_child(TreemapItem::new("产品经理".to_string(), 10.0));
    product_dept.add_child(TreemapItem::new("UI设计".to_string(), 8.0));
    product_dept.add_child(TreemapItem::new("用户研究".to_string(), 5.0));
    
    // 运营部门
    let mut ops_dept = TreemapItem::new("运营部".to_string(), 0.0);
    ops_dept.add_child(TreemapItem::new("市场推广".to_string(), 12.0));
    ops_dept.add_child(TreemapItem::new("客户服务".to_string(), 7.0));
    ops_dept.add_child(TreemapItem::new("商务合作".to_string(), 6.0));

    root.add_child(tech_dept);
    root.add_child(product_dept);
    root.add_child(ops_dept);

    treemap.set_root(root);
    treemap.style_mut().padding = 5.0;
    treemap.style_mut().border_width = 2.0;

    let plot_area = PlotArea::new(50.0, 50.0, 700.0, 500.0);
    scene.add_treemap(treemap, plot_area);

    window.set_scene(scene);
    window_manager.add_window(window);
}

/// 创建平行坐标图演示
fn create_parallel_demo(window_manager: &mut WindowManager) {
    println!("📊 创建平行坐标图演示...");
    
    let mut window = Window::new("平行坐标图演示 - Parallel Coordinates", 1000, 600);
    let mut scene = Scene::new();

    // 创建多维数据 - 汽车性能数据
    let mut parallel = ParallelCoordinates::new()
        .title("汽车性能对比".to_string());

    // 定义维度
    parallel.add_axis("油耗(L/100km)".to_string(), 5.0, 15.0);
    parallel.add_axis("马力(HP)".to_string(), 100.0, 500.0);
    parallel.add_axis("价格(万元)".to_string(), 10.0, 100.0);
    parallel.add_axis("重量(吨)".to_string(), 1.0, 3.0);
    parallel.add_axis("最高时速(km/h)".to_string(), 150.0, 300.0);

    // 添加汽车数据
    let cars = vec![
        ("经济型轿车", vec![6.5, 120.0, 15.0, 1.2, 180.0]),
        ("中级轿车", vec![8.0, 180.0, 25.0, 1.5, 200.0]),
        ("豪华轿车", vec![10.0, 250.0, 50.0, 1.8, 240.0]),
        ("跑车", vec![12.0, 400.0, 80.0, 1.4, 280.0]),
        ("SUV", vec![11.0, 200.0, 35.0, 2.2, 190.0]),
        ("电动车", vec![0.0, 150.0, 30.0, 1.6, 160.0]),
    ];

    for (name, values) in cars {
        parallel.add_data_from_values(name.to_string(), values);
    }

    parallel.style_mut().line_width = 2.0;
    parallel.style_mut().show_grid = true;
    parallel.style_mut().highlight_on_hover = true;

    let plot_area = PlotArea::new(50.0, 50.0, 900.0, 500.0);
    scene.add_parallel_coordinates(parallel, plot_area);

    window.set_scene(scene);
    window_manager.add_window(window);
}
