use vizuara_easy::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建科学论文风格的图形
    let mut fig = figure(1000.0, 700.0);
    fig.title("科学数据可视化示例")
        .scientific_style(); // 应用科学论文风格
    
    // 单个大图显示
    fig.subplot_full()
        .subplot_title("实验数据分析")
        .xlabel("时间 (秒)")
        .ylabel("信号强度 (mV)")
        .xlim(0.0, 20.0)
        .ylim(-1.5, 1.5);
    
    // 生成模拟实验数据
    let experimental_data: Vec<(f32, f32)> = (0..200).map(|i| {
        let t = i as f32 * 0.1;
        let signal = 0.8 * (t * 0.5).sin() * (-t * 0.02).exp() + 
                    0.1 * (t * 3.0).sin() + 
                    0.05 * (t * 10.0).cos();
        // 添加一些噪声
        let noise = (i as f32 * 123.0).sin() * 0.05;
        (t, signal + noise)
    }).collect();
    
    // 理论曲线
    let theoretical_data: Vec<(f32, f32)> = (0..200).map(|i| {
        let t = i as f32 * 0.1;
        let signal = 0.8 * (t * 0.5).sin() * (-t * 0.02).exp();
        (t, signal)
    }).collect();
    
    // 使用科学配色
    let colors = Colors::scientific_sequence();
    
    // 绘制实验数据点和理论曲线
    fig.scatter(&experimental_data, colors[0], 3.0)
        .plot(&theoretical_data, colors[1], 3.0);
    
    // 显示图形
    fig.show()?;
    
    Ok(())
}
