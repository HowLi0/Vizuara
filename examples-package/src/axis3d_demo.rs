//! 3D坐标轴演示 - MATLAB风格
//! 
//! 展示如何在3D场景中显示MATLAB风格的坐标轴，用于科学计算数据可视化

use nalgebra::Point3;
use std::sync::Arc;
use vizuara_3d::{CoordinateSystem3D, GridType, Material};
use vizuara_core::Result;
use vizuara_wgpu::{Vertex3DLit, Wgpu3DLitRenderer};
use winit::{
    event::{Event, WindowEvent, MouseButton, ElementState},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};

struct AppState {
    show_axes: bool,
    current_grid_type: GridType,
    show_planes: bool,
    show_box: bool,
    show_tick_labels: bool,
    show_axis_titles: bool,
    mouse_pressed: bool,
    last_mouse_pos: Option<(f64, f64)>,
}

impl AppState {
    fn new() -> Self {
        Self {
            show_axes: true,
            current_grid_type: GridType::Major,
            show_planes: true,
            show_box: true,
            show_tick_labels: true,
            show_axis_titles: true,
            mouse_pressed: false,
            last_mouse_pos: None,
        }
    }
}

async fn run() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();
    
    // 创建窗口
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("3D坐标轴演示 - MATLAB风格科学计算可视化")
            .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
            .build(&event_loop)
            .unwrap(),
    );

    let size = window.inner_size();

    // 创建渲染器
    let (mut renderer, surface) = Wgpu3DLitRenderer::new(&window, size).await?;

    // 设置更好的初始相机位置
    renderer.set_camera_position(Point3::new(6.0, 6.0, 6.0));

    // 生成更丰富的示例数据点（螺旋线和散点）
    let mut data_points = Vec::new();
    
    // 螺旋线数据
    for i in 0..100 {
        let t = i as f32 * 0.1;
        let radius = 1.5;
        let x = radius * t.cos();
        let y = radius * t.sin();
        let z = t * 0.3 - 1.5;
        if z <= 1.5 {
            data_points.push(Point3::new(x, y, z));
        }
    }

    // 随机散点数据
    for i in 0..30 {
        let angle = i as f32 * 0.2;
        let x = 2.0 * angle.sin() * 0.5;
        let y = 2.0 * angle.cos() * 0.5;
        let z = (i as f32 / 30.0 - 0.5) * 3.0;
        data_points.push(Point3::new(x, y, z));
    }

    // 创建MATLAB风格的3D坐标系统
    let mut coordinate_system = CoordinateSystem3D::new(
        (-3.0, 3.0),   // X范围
        (-3.0, 3.0),   // Y范围
        (-2.0, 2.0),   // Z范围
        Point3::new(-3.0, -3.0, -2.0), // 原点
        1.0,           // 缩放
    )
    .axis_titles("X轴 (m)", "Y轴 (m)", "Z轴 (t)")
    .grid(GridType::Major)
    .show_planes(true)
    .show_box(true)
    .show_tick_labels(true)
    .show_axis_titles(true)
    .tick_count(6, 4) // 6个主刻度，4个次刻度
    .plane_alpha(0.15);

    let mut app_state = AppState::new();

    println!("🎯 3D坐标轴演示启动 - MATLAB风格");
    println!("📋 控制说明:");
    println!("   空格键: 切换坐标轴显示");
    println!("   G键: 切换网格类型 (无/主要/主要+次要)");
    println!("   P键: 切换坐标面显示");
    println!("   B键: 切换坐标轴盒子显示");
    println!("   T键: 切换刻度标签显示");
    println!("   L键: 切换轴标题显示");
    println!("   鼠标左键拖拽: 旋转相机 (轨道控制)");
    println!("   滚轮: 缩放相机");
    println!("   R键: 重置相机到初始位置");
    println!("   ESC键: 退出程序");
    println!();
    
    // 显示坐标轴信息
    println!("📊 3D坐标轴配置:");
    println!("   X轴范围: -3.0 到 3.0 (标题: X轴 (m))");
    println!("   Y轴范围: -3.0 到 3.0 (标题: Y轴 (m))");
    println!("   Z轴范围: -2.0 到 2.0 (标题: Z轴 (t))");
    println!("   🎨 螺旋线数据: 彩虹色谱显示");
    println!("   🔸 散点数据: 基于Z坐标着色");
    println!("   ✨ 3D文本渲染: 刻度标签和轴标题已启用");

    let window_clone = Arc::clone(&window);
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                
                WindowEvent::Resized(physical_size) => {
                    renderer.resize(physical_size, &surface);
                }
                
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        if let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key {
                            match keycode {
                                KeyCode::Space => {
                                    app_state.show_axes = !app_state.show_axes;
                                    println!("🔄 坐标轴显示: {}", if app_state.show_axes { "开启" } else { "关闭" });
                                }
                                KeyCode::KeyG => {
                                    app_state.current_grid_type = match app_state.current_grid_type {
                                        GridType::None => {
                                            println!("📊 网格类型: 主要网格");
                                            GridType::Major
                                        },
                                        GridType::Major => {
                                            println!("📊 网格类型: 主要+次要网格");
                                            GridType::MajorMinor
                                        },
                                        GridType::MajorMinor => {
                                            println!("📊 网格类型: 无网格");
                                            GridType::None
                                        },
                                    };
                                    coordinate_system = coordinate_system.clone().grid(app_state.current_grid_type);
                                }
                                KeyCode::KeyP => {
                                    app_state.show_planes = !app_state.show_planes;
                                    coordinate_system = coordinate_system.clone().show_planes(app_state.show_planes);
                                    println!("🔄 坐标面显示: {}", if app_state.show_planes { "开启" } else { "关闭" });
                                }
                                KeyCode::KeyB => {
                                    app_state.show_box = !app_state.show_box;
                                    coordinate_system = coordinate_system.clone().show_box(app_state.show_box);
                                    println!("🔄 坐标轴盒子: {}", if app_state.show_box { "开启" } else { "关闭" });
                                }
                                KeyCode::KeyT => {
                                    app_state.show_tick_labels = !app_state.show_tick_labels;
                                    coordinate_system = coordinate_system.clone().show_tick_labels(app_state.show_tick_labels);
                                    println!("🔄 刻度标签: {}", if app_state.show_tick_labels { "开启" } else { "关闭" });
                                    
                                    // 显示刻度信息
                                    if app_state.show_tick_labels {
                                        println!("📊 3D刻度标签已在场景中显示:");
                                        println!("   X轴刻度: -3.0, -1.8, -0.6, 0.6, 1.8, 3.0");
                                        println!("   Y轴刻度: -3.0, -1.8, -0.6, 0.6, 1.8, 3.0");
                                        println!("   Z轴刻度: -2.0, -1.2, -0.4, 0.4, 1.2, 2.0");
                                        println!("   ✨ 3D文本渲染已启用，可在场景中查看");
                                    }
                                }
                                KeyCode::KeyL => {
                                    app_state.show_axis_titles = !app_state.show_axis_titles;
                                    coordinate_system = coordinate_system.clone().show_axis_titles(app_state.show_axis_titles);
                                    println!("🔄 轴标题: {}", if app_state.show_axis_titles { "开启" } else { "关闭" });
                                    
                                    // 显示轴标题信息
                                    if app_state.show_axis_titles {
                                        println!("📋 3D轴标题已在场景中显示:");
                                        println!("   X轴: X轴 (m) - 表示空间中的X方向位置");
                                        println!("   Y轴: Y轴 (m) - 表示空间中的Y方向位置");
                                        println!("   Z轴: Z轴 (t) - 表示时间或第三维度");
                                        println!("   ✨ 3D文本渲染已启用，可在场景中查看");
                                    }
                                }
                                KeyCode::KeyR => {
                                    renderer.reset_camera();
                                    renderer.set_camera_position(Point3::new(6.0, 6.0, 6.0));
                                    println!("🔄 相机已重置");
                                }
                                KeyCode::Escape => {
                                    elwt.exit();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                
                WindowEvent::MouseInput { button, state, .. } => {
                    if button == MouseButton::Left {
                        app_state.mouse_pressed = state == ElementState::Pressed;
                        if !app_state.mouse_pressed {
                            app_state.last_mouse_pos = None;
                        }
                    }
                }
                
                WindowEvent::CursorMoved { position, .. } => {
                    if app_state.mouse_pressed {
                        if let Some((last_x, last_y)) = app_state.last_mouse_pos {
                            let delta_x = (position.x - last_x) as f32;
                            let delta_y = (position.y - last_y) as f32;
                            
                            // 改进的相机旋转控制 - 更像MATLAB
                            renderer.rotate_camera(
                                delta_x * 0.008,  // 水平旋转
                                -delta_y * 0.008  // 垂直旋转（反向）
                            );
                        }
                        app_state.last_mouse_pos = Some((position.x, position.y));
                    }
                }
                
                WindowEvent::MouseWheel { delta, .. } => {
                    let zoom_factor = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                            if y > 0.0 { 0.9 } else { 1.1 }
                        }
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            if pos.y > 0.0 { 0.95 } else { 1.05 }
                        }
                    };
                    renderer.zoom_camera(zoom_factor);
                }
                
                WindowEvent::RedrawRequested => {
                    let size = window_clone.inner_size();
                    let aspect_ratio = size.width as f32 / size.height as f32;

                    // 创建数据可视化网格
                    let (vertices, indices) = create_enhanced_data_mesh(&data_points);
                    let material = Material::data_visualization()[0].clone();
                    let objects = vec![(vertices, indices, material)];

                    // 根据设置选择渲染方法
                    let result = if app_state.show_axes {
                        renderer.render_with_axes(
                            &surface,
                            &objects,
                            &coordinate_system,
                            aspect_ratio,
                        )
                    } else {
                        renderer.render_multiple(&surface, &objects, aspect_ratio)
                    };

                    if let Err(e) = result {
                        eprintln!("❌ 渲染错误: {}", e);
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window_clone.request_redraw();
            }
            _ => {}
        }
    }).unwrap();

    Ok(())
}

fn create_enhanced_data_mesh(data_points: &[Point3<f32>]) -> (Vec<Vertex3DLit>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 为每个数据点创建一个更精细的球体
    for (i, point) in data_points.iter().enumerate() {
        let size = if i < 100 { 0.08 } else { 0.12 }; // 螺旋线较小，散点较大
        
        // 根据位置和索引生成颜色
        let color = if i < 100 {
            // 螺旋线：彩虹色谱
            let hue = (i as f32 / 100.0) * 360.0;
            hsv_to_rgb(hue, 0.8, 0.9)
        } else {
            // 散点：根据Z坐标着色
            let z_norm = (point.z + 2.0) / 4.0;
            [1.0 - z_norm, z_norm, 0.5]
        };

        // 创建简化的球体（8面体）
        let sphere_vertices = [
            [0.0, size, 0.0],      // 顶点
            [size, 0.0, 0.0],      // 右
            [0.0, 0.0, size],      // 前
            [-size, 0.0, 0.0],     // 左
            [0.0, 0.0, -size],     // 后
            [0.0, -size, 0.0],     // 底点
        ];

        // 球体三角形面
        let sphere_indices = [
            // 上半球
            0, 1, 2,  0, 2, 3,  0, 3, 4,  0, 4, 1,
            // 下半球
            5, 2, 1,  5, 3, 2,  5, 4, 3,  5, 1, 4,
        ];

        let base_index = vertices.len() as u16;

        // 添加顶点
        for &vertex in &sphere_vertices {
            let world_pos = [
                point.x + vertex[0],
                point.y + vertex[1],
                point.z + vertex[2],
            ];
            
            // 计算法向量（从球心指向表面）
            let normal = [
                vertex[0] / size,
                vertex[1] / size,
                vertex[2] / size,
            ];

            vertices.push(Vertex3DLit {
                position: world_pos,
                normal,
                color,
            });
        }

        // 添加索引
        for &index in &sphere_indices {
            indices.push(base_index + index);
        }
    }

    (vertices, indices)
}

// HSV到RGB颜色转换
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [r + m, g + m, b + m]
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("❌ 应用程序错误: {}", e);
        std::process::exit(1);
    }
}
