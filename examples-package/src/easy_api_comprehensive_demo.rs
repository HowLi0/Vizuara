use vizuara_easy::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建大尺寸图形来容纳更多内容
    let mut fig = figure_large();
    fig.title("Vizuara 2D可视化综合演示");
    
    // 设置3x2的网格布局
    fig.grid(3, 2);
    
    // 1. 多条线图演示
    fig.next_subplot()
        .subplot_title("多条线图")
        .xlabel("X")
        .ylabel("Y")
        .xlim(0.0, 10.0)
        .ylim(-2.0, 2.0);
    
    let datasets = [
        (&testdata::sine_wave(100, 1.0, 1.0), Colors::BLUE, "sin(x)"),
        (&testdata::cosine_wave(100, 1.0, 1.0), Colors::RED, "cos(x)"),
        (&testdata::sine_wave(100, 0.5, 2.0), Colors::GREEN, "0.5*sin(2x)"),
    ];
    fig.multiplot(&datasets, 2.0);
    
    // 2. 箱线图演示
    fig.next_subplot()
        .subplot_title("箱线图");
    
    let box_data = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 3.5, 4.5, 2.5, 5.5],
        vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 4.5, 5.5, 3.5, 6.5],
        vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 4.0, 5.0, 3.0, 6.0],
    ];
    let box_labels = ["组A", "组B", "组C"];
    fig.boxplot(&box_data, &box_labels);
    
    // 3. 饼图演示
    fig.next_subplot()
        .subplot_title("饼图示例");
    
    let pie_data = [
        ("产品A", 30.0),
        ("产品B", 25.0),
        ("产品C", 20.0),
        ("产品D", 15.0),
        ("产品E", 10.0),
    ];
    let pie_colors = Colors::default_sequence();
    fig.pie(&pie_data, &pie_colors);
    
    // 4. 热力图演示
    fig.next_subplot()
        .subplot_title("热力图");
    
    let heatmap_data = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2.0, 4.0, 6.0, 8.0],
        vec![3.0, 6.0, 9.0, 12.0],
        vec![4.0, 8.0, 12.0, 16.0],
    ];
    let x_labels = ["X1", "X2", "X3", "X4"];
    let y_labels = ["Y1", "Y2", "Y3", "Y4"];
    fig.heatmap(&heatmap_data, &x_labels, &y_labels);
    
    // 5. 密度图演示
    fig.next_subplot()
        .subplot_title("密度图");
    
    let density_data: Vec<f32> = (0..1000).map(|i| {
        let x = i as f32 / 100.0;
        x * x.sin() + x.cos() / 2.0
    }).collect();
    fig.density(&density_data, Colors::PURPLE);
    
    // 6. 雷达图演示
    fig.next_subplot()
        .subplot_title("雷达图");
    
    let radar_data = [8.0, 6.0, 7.0, 9.0, 5.0, 8.0];
    let radar_labels = ["能力A", "能力B", "能力C", "能力D", "能力E", "能力F"];
    fig.radar(&radar_data, &radar_labels, Colors::ORANGE);
    
    // 显示图形
    fig.show()?;
    
    Ok(())
}
