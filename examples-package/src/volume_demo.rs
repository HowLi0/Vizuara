//! 体积渲染演示
//!
//! 展示3D体积数据的可视化技术

use std::f32::consts::PI;
use vizuara_3d::{TransferFunction, VolumeData, VolumeRenderer};
use vizuara_core::Color;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 启动 Vizuara 体积渲染演示！");

    // 1. 创建3D高斯分布体积数据
    println!("📊 生成3D高斯分布体积数据...");
    let gaussian_volume = VolumeData::from_function(
        ((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)), // 边界
        (64, 64, 64), // 分辨率
        |x, y, z| {
            // 多个高斯峰的叠加
            let gauss1 = (-((x - 0.5).powi(2) + (y - 0.5).powi(2) + (z - 0.5).powi(2)) / 0.5).exp();
            let gauss2 = (-((x + 0.5).powi(2) + (y + 0.5).powi(2) + (z + 0.5).powi(2)) / 0.3).exp();
            let gauss3 = (-((x - 0.0).powi(2) + (y - 0.0).powi(2) + (z + 1.0).powi(2)) / 0.8).exp();
            
            gauss1 + gauss2 * 0.7 + gauss3 * 0.5
        }
    );

    println!("📏 高斯体积: {:?}, 值范围: {:?}", 
        gaussian_volume.dimensions, 
        gaussian_volume.value_range
    );

    // 2. 创建3D噪声体积数据
    println!("🌀 生成3D噪声体积数据...");
    let noise_volume = VolumeData::from_function(
        ((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0)),
        (32, 32, 32),
        |x, y, z| {
            // 简单的3D Perlin噪声近似
            let freq1 = 2.0;
            let freq2 = 4.0;
            let freq3 = 8.0;
            
            let noise1 = ((x * freq1 * PI).sin() * (y * freq1 * PI).cos() * (z * freq1 * PI).sin()).abs();
            let noise2 = ((x * freq2 * PI).cos() * (y * freq2 * PI).sin() * (z * freq2 * PI).cos()).abs() * 0.5;
            let noise3 = ((x * freq3 * PI).sin() * (y * freq3 * PI).sin() * (z * freq3 * PI).sin()).abs() * 0.25;
            
            noise1 + noise2 + noise3
        }
    );

    println!("📏 噪声体积: {:?}, 值范围: {:?}", 
        noise_volume.dimensions, 
        noise_volume.value_range
    );

    // 3. 创建螺旋体积数据
    println!("🌪️  生成3D螺旋体积数据...");
    let spiral_volume = VolumeData::from_function(
        ((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)),
        (48, 48, 48),
        |x, _y, z| {
            let r = (x * x + z * z).sqrt();
            let theta = z.atan2(x);
            let height_factor = (z + 2.0) / 4.0; // 归一化高度
            
            // 创建螺旋密度场
            let spiral_angle = theta + r * 3.0 + height_factor * PI * 4.0;
            let spiral_density = (spiral_angle.sin() * 3.0).exp() * 0.1;
            
            // 径向衰减
            let radial_falloff = (-(r - 1.0).powi(2) / 0.5).exp();
            
            // 高度调制
            let height_modulation = (-((z - 0.0).powi(2)) / 2.0).exp();
            
            spiral_density * radial_falloff * height_modulation
        }
    );

    println!("📏 螺旋体积: {:?}, 值范围: {:?}", 
        spiral_volume.dimensions, 
        spiral_volume.value_range
    );

    // 4. 创建不同的传输函数
    println!("🎨 配置传输函数...");

    // 冷暖色传输函数（适合温度场）
    let thermal_tf = TransferFunction::new()
        .add_control_point(0.0, Color::rgb(0.0, 0.0, 0.5), 0.0)    // 深蓝，透明
        .add_control_point(0.2, Color::rgb(0.0, 0.5, 1.0), 0.3)    // 浅蓝，微透明
        .add_control_point(0.4, Color::rgb(0.0, 1.0, 1.0), 0.5)    // 青色，半透明
        .add_control_point(0.6, Color::rgb(1.0, 1.0, 0.0), 0.7)    // 黄色，较不透明
        .add_control_point(0.8, Color::rgb(1.0, 0.5, 0.0), 0.9)    // 橙色，不透明
        .add_control_point(1.0, Color::rgb(1.0, 0.0, 0.0), 1.0);   // 红色，完全不透明

    // 等离子体传输函数
    let plasma_tf = TransferFunction::new()
        .add_control_point(0.0, Color::rgb(0.2, 0.0, 0.4), 0.0)    // 深紫，透明
        .add_control_point(0.3, Color::rgb(0.8, 0.0, 0.8), 0.4)    // 紫红，半透明
        .add_control_point(0.6, Color::rgb(1.0, 0.4, 0.6), 0.7)    // 粉色，较不透明
        .add_control_point(0.9, Color::rgb(1.0, 1.0, 0.8), 1.0);   // 亮黄，完全不透明

    // 医学成像传输函数
    let medical_tf = TransferFunction::new()
        .add_control_point(0.0, Color::rgb(0.0, 0.0, 0.0), 0.0)    // 黑色，透明
        .add_control_point(0.1, Color::rgb(0.3, 0.1, 0.1), 0.1)    // 深红，微透明
        .add_control_point(0.5, Color::rgb(0.8, 0.6, 0.4), 0.6)    // 肉色，半透明
        .add_control_point(0.8, Color::rgb(1.0, 0.9, 0.8), 0.9)    // 骨白，不透明
        .add_control_point(1.0, Color::rgb(1.0, 1.0, 1.0), 1.0);   // 纯白，完全不透明

    // 5. 创建体积渲染器
    println!("🔬 创建体积渲染器...");

    let gaussian_renderer = VolumeRenderer::new(gaussian_volume)
        .transfer_function(thermal_tf.clone())
        .sampling(0.02, 300);

    let noise_renderer = VolumeRenderer::new(noise_volume)
        .transfer_function(plasma_tf.clone())
        .sampling(0.025, 250);

    let spiral_renderer = VolumeRenderer::new(spiral_volume)
        .transfer_function(medical_tf.clone())
        .sampling(0.015, 400);

    println!("✅ 体积渲染器创建完成！");

    // 6. 演示光线投射
    println!("🔫 执行光线投射演示...");

    let test_rays = vec![
        (nalgebra::Point3::new(-3.0, 0.0, 0.0), nalgebra::Vector3::new(1.0, 0.0, 0.0)),
        (nalgebra::Point3::new(0.0, -3.0, 0.0), nalgebra::Vector3::new(0.0, 1.0, 0.0)),
        (nalgebra::Point3::new(0.0, 0.0, -3.0), nalgebra::Vector3::new(0.0, 0.0, 1.0)),
        (nalgebra::Point3::new(-2.0, -2.0, -2.0), nalgebra::Vector3::new(1.0, 1.0, 1.0)),
    ];

    for (i, (origin, direction)) in test_rays.iter().enumerate() {
        // 高斯体积渲染
        let gauss_color = gaussian_renderer.ray_cast(*origin, *direction);
        println!("🌈 光线 {}: 高斯体积 -> RGBA({:.3}, {:.3}, {:.3}, {:.3})", 
            i + 1, gauss_color.r, gauss_color.g, gauss_color.b, gauss_color.a);

        // 噪声体积渲染
        let noise_color = noise_renderer.ray_cast(*origin, *direction);
        println!("🌈 光线 {}: 噪声体积 -> RGBA({:.3}, {:.3}, {:.3}, {:.3})", 
            i + 1, noise_color.r, noise_color.g, noise_color.b, noise_color.a);

        // 螺旋体积渲染
        let spiral_color = spiral_renderer.ray_cast(*origin, *direction);
        println!("🌈 光线 {}: 螺旋体积 -> RGBA({:.3}, {:.3}, {:.3}, {:.3})", 
            i + 1, spiral_color.r, spiral_color.g, spiral_color.b, spiral_color.a);
    }

    // 7. 展示传输函数采样
    println!("🎨 传输函数采样测试...");
    let test_values = [0.0, 0.25, 0.5, 0.75, 1.0];
    
    for value in test_values {
        let (thermal_color, thermal_alpha) = thermal_tf.sample(value);
        println!("🔥 热力传输函数 {:.2} -> RGB({:.3}, {:.3}, {:.3}), α={:.3}",
            value, thermal_color.r, thermal_color.g, thermal_color.b, thermal_alpha);
    }

    println!("✅ 体积渲染演示完成！");
    println!("");
    println!("📚 体积渲染功能说明:");
    println!("   📊 支持3D标量场数据可视化");
    println!("   🎨 可配置的传输函数");
    println!("   🔫 光线投射渲染算法");
    println!("   🔬 三线性插值采样");
    println!("   🌈 Alpha混合合成");
    println!("   ⚡ 早期光线终止优化");
    println!("");
    println!("🚀 应用领域:");
    println!("   🏥 医学成像 (CT/MRI)");
    println!("   🌊 流体仿真可视化");
    println!("   🔬 科学数据分析");
    println!("   🌋 地质数据展示");
    println!("   ☁️  气象数据可视化");

    Ok(())
}
