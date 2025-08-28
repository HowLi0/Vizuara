use std::f32::consts::PI;
use vizuara_3d::{Mesh3D, Scatter3D, Surface3D};
use vizuara_core::Color;
use vizuara_window::Window3D;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 启动 Vizuara 真实3D窗口演示！");

    // 创建3D散点数据 - 螺旋
    println!("📈 生成3D螺旋散点数据...");
    let mut scatter_data = Vec::new();
    for i in 0..100 {
        let t = i as f32 * 0.2;
        let x = t.cos() * (1.0 + t * 0.05);
        let y = t.sin() * (1.0 + t * 0.05);
        let z = t * 0.1;
        scatter_data.push((x, y, z));
    }

    let scatter3d = Scatter3D::from_data(&scatter_data)
        .default_color(Color::rgb(1.0, 0.3, 0.3))
        .default_size(6.0);

    println!("✨ 散点图: {} 个点", scatter3d.point_count());

    // 创建3D表面 - 波纹函数
    println!("🌊 生成3D波纹表面...");
    let surface = Surface3D::from_function(
        (-3.0, 3.0), // X范围
        (-3.0, 3.0), // Y范围
        (50, 50),    // 高分辨率
        |x, y| {
            let r = (x * x + y * y).sqrt();
            if r == 0.0 {
                1.0
            } else {
                (r * PI).sin() / r * 0.5
            }
        },
    )
    .wireframe(true)
    .wireframe_color(Color::rgb(0.2, 0.8, 0.2));

    println!(
        "🕸️  表面网格: {}x{}",
        surface.mesh().width,
        surface.mesh().height
    );

    // 创建3D立方体网格
    println!("📦 创建3D立方体...");
    let cube = Mesh3D::cube(0.8);
    println!("🔺 立方体: {} 个三角形", cube.triangle_count());

    // 创建并运行3D窗口
    println!("🎮 启动交互式3D窗口...");
    println!("💡 操作说明:");
    println!("   🖱️  左键拖拽 - 旋转视角");
    println!("   🎱 滚轮 - 缩放场景");
    println!("   ⌨️  R键 - 重置相机");
    println!("   ⌨️  Esc键 - 退出程序");

    let window = Window3D::new()
        .add_scatter3d(scatter3d)
        .add_surface3d(surface)
        .add_mesh3d(cube);

    window.run().await?;

    println!("✅ 3D可视化演示完成！");
    Ok(())
}
