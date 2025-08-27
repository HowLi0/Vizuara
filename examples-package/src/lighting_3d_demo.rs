//! 光照3D可视化演示
//! 
//! 展示带有物理材质和光照的高级3D渲染

use std::error::Error;
use winit::{
    event::{Event, WindowEvent, ElementState, KeyEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    keyboard::{PhysicalKey, KeyCode},
};
use std::sync::Arc;
use vizuara_3d::{Light, Material};
use vizuara_wgpu::{Wgpu3DLitRenderer, Vertex3DLit};
use vizuara_core::{Color, VizuaraError};
use nalgebra::{Point3, Vector3};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌟 启动 Vizuara 光照3D演示！");
    
    // 创建3D场景和材质
    println!("🎨 准备3D场景和材质...");
    
    // 1. 金属球体 (散点云)
    let metal_vertices = generate_sphere_vertices(Point3::new(-3.0, 0.0, 0.0), 1.0, 20);
    let metal_material = Material::metal(Color::rgb(0.8, 0.8, 0.9));
    
    // 2. 塑料表面
    let plastic_vertices = generate_surface_vertices();
    let plastic_material = Material::plastic(Color::rgb(1.0, 0.4, 0.4));
    
    // 3. 玻璃立方体
    let glass_vertices = generate_cube_vertices([3.0, 0.0, 0.0], 1.5);
    let glass_material = Material::glass(Color::rgb(0.9, 0.95, 1.0));
    
    // 创建高级光照场景
    let lights = vec![
        // 主光源 - 暖色调
        Light::directional(
            Vector3::new(-0.5, -0.8, -0.3),
            Color::rgb(1.0, 0.95, 0.8),
            3.0
        ),
        // 环境填充光 - 冷色调
        Light::directional(
            Vector3::new(0.3, 0.4, 0.7),
            Color::rgb(0.6, 0.8, 1.0),
            1.2
        ),
        // 背光 - 轮廓光
        Light::directional(
            Vector3::new(0.8, 0.2, 0.6),
            Color::rgb(1.0, 0.7, 0.9),
            0.8
        ),
        // 点光源 - 动态高亮
        Light::point(
            Point3::new(0.0, 4.0, 0.0),
            Color::rgb(1.0, 1.0, 0.8),
            2.0,
            10.0
        ),
    ];
    
    println!("💡 配置了 {} 个光源", lights.len());
    println!("🔮 材质类型: 金属、塑料、玻璃");
    
    // 启动光照3D窗口
    run_lit_3d_window(
        vec![
            (metal_vertices, metal_material),
            (plastic_vertices, plastic_material),
            (glass_vertices, glass_material),
        ],
        lights
    ).await?;
    
    println!("✅ 光照3D演示完成！");
    Ok(())
}

/// 生成球体顶点
fn generate_sphere_vertices(center: Point3<f32>, radius: f32, subdivisions: usize) -> Vec<Vertex3DLit> {
    let mut vertices = Vec::new();
    
    for i in 0..subdivisions {
        for j in 0..(subdivisions * 2) {
            let phi = std::f32::consts::PI * (i as f32) / (subdivisions as f32);
            let theta = 2.0 * std::f32::consts::PI * (j as f32) / (subdivisions as f32 * 2.0);
            
            let x = center.x + radius * phi.sin() * theta.cos();
            let y = center.y + radius * phi.sin() * theta.sin();
            let z = center.z + radius * phi.cos();
            
            // 球面法向量 (从中心指向表面)
            let _normal = [(x - center.x)/radius, (y - center.y)/radius, (z - center.z)/radius];
            let color = [0.8, 0.8, 0.9]; // 金属色
            
            // 为每个点创建一个小立方体
            let cube_size = 0.1;
            add_cube_vertices(&mut vertices, [x, y, z], cube_size, color);
        }
    }
    
    vertices
}

/// 生成表面顶点
fn generate_surface_vertices() -> Vec<Vertex3DLit> {
    let mut vertices = Vec::new();
    let resolution = 40;
    
    for i in 0..resolution {
        for j in 0..resolution {
            let x1 = -2.0 + (4.0 * i as f32) / resolution as f32;
            let y1 = -2.0 + (4.0 * j as f32) / resolution as f32;
            let x2 = -2.0 + (4.0 * (i + 1) as f32) / resolution as f32;
            let y2 = -2.0 + (4.0 * (j + 1) as f32) / resolution as f32;
            
            let z1 = 0.5 * (x1*x1 + y1*y1).sin();
            let z2 = 0.5 * (x2*x2 + y1*y1).sin();
            let z3 = 0.5 * (x1*x1 + y2*y2).sin();
            let z4 = 0.5 * (x2*x2 + y2*y2).sin();
            
            // 计算表面法向量 (简化)
            let dx = 1.0 * (x1*x1 + y1*y1).cos() * 2.0 * x1;
            let dy = 1.0 * (x1*x1 + y1*y1).cos() * 2.0 * y1;
            let normal_len = (dx*dx + dy*dy + 1.0).sqrt();
            let normal = [-dx/normal_len, -dy/normal_len, 1.0/normal_len];
            
            // 基于高度的颜色
            let t = (z1 + 1.0) * 0.5;
            let color = [1.0 - t, 0.4, t];
            
            // 创建两个三角形
            vertices.extend_from_slice(&[
                Vertex3DLit { position: [x1, y1, z1], normal, color },
                Vertex3DLit { position: [x2, y1, z2], normal, color },
                Vertex3DLit { position: [x1, y2, z3], normal, color },
                
                Vertex3DLit { position: [x2, y1, z2], normal, color },
                Vertex3DLit { position: [x2, y2, z4], normal, color },
                Vertex3DLit { position: [x1, y2, z3], normal, color },
            ]);
        }
    }
    
    vertices
}

/// 生成立方体顶点
fn generate_cube_vertices(center: [f32; 3], size: f32) -> Vec<Vertex3DLit> {
    let mut vertices = Vec::new();
    let color = [0.9, 0.95, 1.0]; // 玻璃色
    add_cube_vertices(&mut vertices, center, size, color);
    vertices
}

/// 添加立方体顶点到列表
fn add_cube_vertices(vertices: &mut Vec<Vertex3DLit>, center: [f32; 3], size: f32, color: [f32; 3]) {
    let half_size = size * 0.5;
    
    // 定义立方体的8个顶点
    let cube_vertices = [
        [center[0] - half_size, center[1] - half_size, center[2] - half_size], // 0
        [center[0] + half_size, center[1] - half_size, center[2] - half_size], // 1
        [center[0] + half_size, center[1] + half_size, center[2] - half_size], // 2
        [center[0] - half_size, center[1] + half_size, center[2] - half_size], // 3
        [center[0] - half_size, center[1] - half_size, center[2] + half_size], // 4
        [center[0] + half_size, center[1] - half_size, center[2] + half_size], // 5
        [center[0] + half_size, center[1] + half_size, center[2] + half_size], // 6
        [center[0] - half_size, center[1] + half_size, center[2] + half_size], // 7
    ];
    
    // 定义6个面，每个面有2个三角形
    let faces = [
        // 前面 (z+)
        ([4, 5, 6], [0.0, 0.0, 1.0]),
        ([4, 6, 7], [0.0, 0.0, 1.0]),
        // 后面 (z-)
        ([1, 0, 3], [0.0, 0.0, -1.0]),
        ([1, 3, 2], [0.0, 0.0, -1.0]),
        // 右面 (x+)
        ([5, 1, 2], [1.0, 0.0, 0.0]),
        ([5, 2, 6], [1.0, 0.0, 0.0]),
        // 左面 (x-)
        ([0, 4, 7], [-1.0, 0.0, 0.0]),
        ([0, 7, 3], [-1.0, 0.0, 0.0]),
        // 上面 (y+)
        ([3, 7, 6], [0.0, 1.0, 0.0]),
        ([3, 6, 2], [0.0, 1.0, 0.0]),
        // 下面 (y-)
        ([0, 1, 5], [0.0, -1.0, 0.0]),
        ([0, 5, 4], [0.0, -1.0, 0.0]),
    ];
    
    for (triangle, normal) in faces.iter() {
        for &vertex_idx in triangle {
            vertices.push(Vertex3DLit {
                position: cube_vertices[vertex_idx],
                normal: *normal,
                color,
            });
        }
    }
}

/// 运行光照3D窗口
async fn run_lit_3d_window(
    objects: Vec<(Vec<Vertex3DLit>, Material)>,
    lights: Vec<Light>,
) -> Result<(), Box<dyn Error>> {
    println!("🎮 启动光照3D窗口...");
    
    // 创建事件循环
    let event_loop = EventLoop::new()
        .map_err(|e| VizuaraError::RenderError(format!("Failed to create event loop: {}", e)))?;
    
    // 创建窗口
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Vizuara - 光照3D可视化")
            .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
            .build(&event_loop)
            .map_err(|e| VizuaraError::RenderError(format!("Failed to create window: {}", e)))?
    );
    
    let size = window.inner_size();
    println!("✅ 窗口创建成功: {}x{}", size.width, size.height);
    
    // 创建光照渲染器
    let (mut renderer, surface) = Wgpu3DLitRenderer::new(&window, size).await?;
    println!("✅ 光照渲染器初始化成功");
    
    // 配置光源
    for light in lights {
        renderer.add_light(light);
    }
    renderer.set_ambient_light([0.05, 0.05, 0.1], 0.2);
    println!("💡 光照系统配置完成");
    
    // 统计渲染数据
    let total_vertices: usize = objects.iter().map(|(vertices, _)| vertices.len()).sum();
    println!("📐 场景统计: {} 个物体, {} 个顶点", objects.len(), total_vertices);
    
    // 打印操作说明
    println!("💡 操作说明:");
    println!("   🖱️  左键拖拽 - 旋转相机");
    println!("   🎱 滚轮 - 缩放");
    println!("   ⌨️  R键 - 重置相机");
    println!("   ⌨️  Esc键 - 退出");
    
    let window_id = window.id();
    let window_clone = Arc::clone(&window);
    let mut mouse_pressed = false;
    let mut last_mouse_pos: Option<(f64, f64)> = None;
    
    // 主事件循环
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { 
                event, 
                window_id: event_window_id, 
                .. 
            } if event_window_id == window_id => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("🔚 关闭光照3D窗口");
                        control_flow.exit();
                    }
                    
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width > 0 && physical_size.height > 0 {
                            println!("📏 窗口尺寸变更: {}x{}", physical_size.width, physical_size.height);
                            renderer.resize(physical_size, &surface);
                            window_clone.request_redraw();
                        }
                    }
                    
                    WindowEvent::CursorMoved { position, .. } => {
                        if mouse_pressed {
                            if let Some(last_pos) = last_mouse_pos {
                                let delta_x = (position.x - last_pos.0) as f32 * 0.01;
                                let delta_y = (position.y - last_pos.1) as f32 * 0.01;
                                renderer.rotate_camera(delta_x, -delta_y);
                                window_clone.request_redraw();
                            }
                        }
                        last_mouse_pos = Some((position.x, position.y));
                    }
                    
                    WindowEvent::MouseInput { state, button, .. } => {
                        if button == winit::event::MouseButton::Left {
                            mouse_pressed = state == ElementState::Pressed;
                        }
                    }
                    
                    WindowEvent::MouseWheel { delta, .. } => {
                        let zoom_factor = match delta {
                            winit::event::MouseScrollDelta::LineDelta(_, y) => {
                                if y > 0.0 { 0.9 } else { 1.1 }
                            }
                            winit::event::MouseScrollDelta::PixelDelta(pos) => {
                                if pos.y > 0.0 { 0.95 } else { 1.05 }
                            }
                        };
                        renderer.zoom_camera(zoom_factor);
                        window_clone.request_redraw();
                    }
                    
                    WindowEvent::KeyboardInput { 
                        event: KeyEvent { 
                            state: ElementState::Pressed, 
                            physical_key: PhysicalKey::Code(key), 
                            .. 
                        }, 
                        .. 
                    } => {
                        match key {
                            KeyCode::Escape => {
                                println!("🔑 ESC键退出");
                                control_flow.exit();
                            }
                            KeyCode::KeyR => {
                                println!("📷 相机已重置");
                                renderer.reset_camera();
                                window_clone.request_redraw();
                            }
                            _ => {}
                        }
                    }
                    
                    WindowEvent::RedrawRequested => {
                        // 渲染所有物体
                        let aspect_ratio = size.width as f32 / size.height as f32;
                        
                        for (vertices, material) in &objects {
                            let indices: Vec<u16> = (0..vertices.len() as u16).collect();
                            
                            if let Err(e) = renderer.render(
                                &surface, 
                                vertices, 
                                &indices, 
                                material, 
                                aspect_ratio
                            ) {
                                eprintln!("❌ 渲染错误: {}", e);
                            }
                        }
                    }
                    
                    _ => {}
                }
            }
            
            Event::AboutToWait => {
                window_clone.request_redraw();
            }
            
            _ => {}
        }
    })
    .map_err(|e| VizuaraError::RenderError(format!("Event loop error: {}", e)))?;
    
    Ok(())
}
