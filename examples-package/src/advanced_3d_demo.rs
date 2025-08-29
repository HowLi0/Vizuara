//! 高级3D可视化演示
//! 
//! 展示新增的几何体类型和高级3D功能

use std::f32::consts::PI;
use vizuara_3d::{Camera3D, Mesh3D, Plot3DArea, Scatter3D, Surface3D};
use vizuara_core::Color;
use vizuara_window::Window3D;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 启动 Vizuara 高级3D可视化演示！");

    // 创建多种几何体
    println!("🔺 生成高级几何体...");
    
    // 1. 高质量球体
    let sphere = Mesh3D::sphere(1.2, 16);
    println!("🌍 球体: {} 个三角形", sphere.triangle_count());

    // 2. 圆柱体
    let cylinder = Mesh3D::cylinder(0.8, 2.0, 12);
    println!("🏗️  圆柱体: {} 个三角形", cylinder.triangle_count());

    // 3. 圆锥体
    let cone = Mesh3D::cone(1.0, 1.5, 10);
    println!("🍦 圆锥体: {} 个三角形", cone.triangle_count());

    // 4. 环面（甜甜圈）
    let torus = Mesh3D::torus(1.5, 0.5, 16, 12);
    println!("🍩 环面: {} 个三角形", torus.triangle_count());

    // 创建复杂的3D数据可视化
    println!("📊 生成复杂数据可视化...");

    // 3D参数曲面 - 克莱因瓶投影
    let klein_surface = Surface3D::from_function(
        (0.0, 2.0 * PI), // U参数范围
        (0.0, 2.0 * PI), // V参数范围
        (40, 30),        // 高分辨率
        |u, v| {
            // 克莱因瓶的简化版本（投影到3D）
            let cos_u = u.cos();
            let sin_u = u.sin();
            let cos_v = v.cos();
            
            // Z坐标作为高度
            sin_u * cos_v * 0.5 + cos_u * (v / PI - 1.0) * 0.3
        },
    )
    .wireframe(true)
    .wireframe_color(Color::rgb(0.8, 0.3, 0.8));

    println!("🌊 参数曲面: {}x{}", 
        klein_surface.mesh().width, 
        klein_surface.mesh().height
    );

    // 多层3D散点数据 - DNA双螺旋结构
    println!("🧬 生成DNA螺旋结构...");
    let mut dna_data = Vec::new();
    
    for i in 0..200 {
        let t = i as f32 * 0.1;
        let height = t * 0.05;
        
        // 第一条螺旋链
        let x1 = (t * 2.0).cos() * 1.0;
        let z1 = (t * 2.0).sin() * 1.0;
        dna_data.push((x1, height, z1));
        
        // 第二条螺旋链（相位差π）
        let x2 = (t * 2.0 + PI).cos() * 1.0;
        let z2 = (t * 2.0 + PI).sin() * 1.0;
        dna_data.push((x2, height, z2));
        
        // 连接链（每10个点添加）
        if i % 10 == 0 {
            for j in 1..=3 {
                let factor = j as f32 / 4.0;
                let x_bridge = x1 * (1.0 - factor) + x2 * factor;
                let z_bridge = z1 * (1.0 - factor) + z2 * factor;
                dna_data.push((x_bridge, height, z_bridge));
            }
        }
    }

    let dna_scatter = Scatter3D::from_data(&dna_data)
        .default_color(Color::rgb(0.2, 0.8, 0.9))
        .default_size(4.0);

    println!("🧬 DNA结构: {} 个数据点", dna_scatter.point_count());

    // 创建综合3D场景
    println!("🎮 启动高级3D可视化窗口...");
    println!("💡 新功能展示:");
    println!("   🌍 高质量球体 (UV球)");
    println!("   🏗️  圆柱体和圆锥体");
    println!("   🍩 环面（甜甜圈）几何体");
    println!("   🌊 参数曲面可视化");
    println!("   🧬 复杂数据结构（DNA螺旋）");
    println!("   🎮 改进的交互控制");
    println!("");
    println!("💻 控制说明:");
    println!("   🖱️  左键拖拽 - 轨道旋转");
    println!("   🎱 滚轮 - 缩放场景");
    println!("   ⌨️  R键 - 重置相机");
    println!("   ⌨️  Esc键 - 退出");

    // 启动窗口（暂时只显示一个几何体，之后可以扩展到多对象）
    let window = Window3D::new()
        .add_scatter3d(dna_scatter)
        .add_surface3d(klein_surface)
        .add_mesh3d(torus);  // 显示环面作为主要几何体

    window.run().await?;

    println!("✅ 高级3D可视化演示完成！");
    Ok(())
}
