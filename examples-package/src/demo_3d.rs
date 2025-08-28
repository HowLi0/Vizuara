use std::f32::consts::PI;
use vizuara_3d::{Camera3D, Mesh3D, Plot3DArea, Scatter3D, Surface3D};
use vizuara_core::Color;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Vizuara 3D 可视化演示启动！");

    // 创建3D散点数据
    println!("📈 生成3D散点数据...");
    let mut scatter_data = Vec::new();

    // 螺旋形数据
    for i in 0..50 {
        let t = i as f32 * 0.3;
        let x = t.cos() * (1.0 + t * 0.1);
        let y = t.sin() * (1.0 + t * 0.1);
        let z = t * 0.2;
        scatter_data.push((x, y, z));
    }

    let scatter3d = Scatter3D::from_data(&scatter_data)
        .default_color(Color::rgb(1.0, 0.3, 0.3))
        .default_size(8.0);

    println!("📊 散点图包含 {} 个点", scatter3d.point_count());

    // 创建3D表面数据 - 数学函数
    println!("🌊 生成3D表面数据...");
    let surface = Surface3D::from_function(
        (-2.0, 2.0), // X范围
        (-2.0, 2.0), // Y范围
        (20, 20),    // 分辨率
        |x, y| {
            // 波纹函数
            let r = (x * x + y * y).sqrt();
            if r == 0.0 {
                1.0
            } else {
                (r * PI).sin() / r
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

    // 创建3D网格 - 立方体
    println!("📦 创建3D立方体网格...");
    let cube_mesh = Mesh3D::cube(1.5);
    println!("🔺 立方体包含 {} 个三角形", cube_mesh.triangle_count());

    // 创建球体网格
    println!("🌍 创建3D球体网格...");
    let sphere_mesh = Mesh3D::sphere(1.0, 0);
    println!("🔺 球体包含 {} 个三角形", sphere_mesh.triangle_count());

    // 设置3D相机
    println!("📷 配置3D相机...");
    let _camera = Camera3D::new()
        .position(5.0, 5.0, 5.0)
        .target(0.0, 0.0, 0.0)
        .fov_degrees(45.0)
        .aspect_ratio(4.0 / 3.0)
        .clip_planes(0.1, 100.0);

    // 创建3D绘图区域
    let plot_area = Plot3DArea::new((-3.0, 3.0), (-3.0, 3.0), (-2.0, 3.0))
        .perspective(4.0 / 3.0, PI / 4.0, 0.1, 100.0)
        .camera(
            nalgebra::Point3::new(5.0, 5.0, 5.0),
            nalgebra::Point3::new(0.0, 0.0, 0.0),
            nalgebra::Vector3::new(0.0, 1.0, 0.0),
        );

    println!("🖼️  生成3D渲染图元...");

    // 生成散点图图元
    let scatter_primitives = scatter3d.generate_primitives(&plot_area);
    println!("✨ 散点图生成了 {} 个图元", scatter_primitives.len());

    // 生成表面图图元
    let surface_primitives = surface.generate_primitives(&plot_area);
    println!("🌊 表面图生成了 {} 个图元", surface_primitives.len());

    // 生成立方体图元
    let cube_primitives = cube_mesh.generate_wireframe_primitives(&plot_area);
    println!("📦 立方体生成了 {} 个图元", cube_primitives.len());

    // 生成球体图元
    let sphere_primitives = sphere_mesh.generate_wireframe_primitives(&plot_area);
    println!("🌍 球体生成了 {} 个图元", sphere_primitives.len());

    // 显示数据统计
    if let Some(bounds) = scatter3d.bounds() {
        println!("📏 散点数据边界:");
        println!("   X: {:.2} ~ {:.2}", bounds.0 .0, bounds.0 .1);
        println!("   Y: {:.2} ~ {:.2}", bounds.1 .0, bounds.1 .1);
        println!("   Z: {:.2} ~ {:.2}", bounds.2 .0, bounds.2 .1);
    }

    let surface_bounds = surface.mesh().bounds();
    println!("📏 表面数据边界:");
    println!(
        "   X: {:.2} ~ {:.2}",
        surface_bounds.0 .0, surface_bounds.0 .1
    );
    println!(
        "   Y: {:.2} ~ {:.2}",
        surface_bounds.1 .0, surface_bounds.1 .1
    );
    println!(
        "   Z: {:.2} ~ {:.2}",
        surface_bounds.2 .0, surface_bounds.2 .1
    );

    if let Some(cube_bounds) = cube_mesh.bounds() {
        println!("📏 立方体边界:");
        println!("   X: {:.2} ~ {:.2}", cube_bounds.0 .0, cube_bounds.0 .1);
        println!("   Y: {:.2} ~ {:.2}", cube_bounds.1 .0, cube_bounds.1 .1);
        println!("   Z: {:.2} ~ {:.2}", cube_bounds.2 .0, cube_bounds.2 .1);
    }

    println!("🎯 3D可视化演示完成！");
    println!(
        "📊 总计生成了 {} 个图形元素",
        scatter_primitives.len()
            + surface_primitives.len()
            + cube_primitives.len()
            + sphere_primitives.len()
    );

    println!("💡 3D功能说明:");
    println!("   • ✨ 3D散点图 - 支持螺旋和复杂数据");
    println!("   • 🌊 3D表面图 - 数学函数可视化");
    println!("   • 📦 3D网格 - 立方体和球体基础形状");
    println!("   • 📷 3D相机 - 透视投影和视图变换");
    println!("   • 🎮 交互控制 - 旋转、缩放、平移 (待实现)");

    Ok(())
}
