use vizuara_easy::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建标准尺寸的图形
    let mut fig = figure_std();
    
    // 设置网格布局：2行2列
    fig.grid(2, 2);
    
    // 第一个子图：线图和散点图的组合
    fig.next_subplot()
        .subplot_title("线图与散点图")
        .xlabel("X轴")
        .ylabel("Y轴");
    
    let sine_data = testdata::sine_wave(100, 1.0, 2.0);
    let cos_data = testdata::cosine_wave(100, 0.8, 2.0);
    let random_data = testdata::random_scatter(30);
    
    fig.plot(&sine_data, Colors::BLUE, 2.0)
        .plot(&cos_data, Colors::RED, 2.0)
        .scatter(&random_data, Colors::GREEN, 4.0);
    
    // 第二个子图：柱状图
    fig.next_subplot()
        .subplot_title("柱状图示例")
        .xlabel("类别")
        .ylabel("数值");
    
    let categories = ["A", "B", "C", "D", "E"];
    let values = [23.0, 45.0, 56.0, 78.0, 32.0];
    fig.bar(&categories, &values, Colors::ORANGE);
    
    // 第三个子图：直方图
    fig.next_subplot()
        .subplot_title("直方图示例")
        .xlabel("数值")
        .ylabel("频次");
    
    let hist_data: Vec<f32> = (0..200).map(|_| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        rand::random::<u64>().hash(&mut hasher);
        let seed = hasher.finish();
        (seed % 100) as f32 / 10.0 // 0-10范围的随机数
    }).collect();
    
    fig.hist(&hist_data, 20, Colors::PURPLE);
    
    // 第四个子图：面积图
    fig.next_subplot()
        .subplot_title("面积图示例")
        .xlabel("时间")
        .ylabel("数值");
    
    let area_data = testdata::linear(50, 0.5, 2.0);
    fig.area(&area_data, Colors::CYAN, 0.6);
    
    // 显示图形
    fig.show()?;
    
    Ok(())
}

// 简单的随机数生成模块（因为没有外部依赖）
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn random<T>() -> T 
    where 
        T: From<u64>
    {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        T::from(hasher.finish())
    }
}
