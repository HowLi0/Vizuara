use nalgebra::Point2;
use std::sync::Arc;
use vizuara_core::{Primitive, Result, Style, VizuaraError};
use vizuara_wgpu::WgpuRenderer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    //window::{Window, WindowBuilder},
    //dpi::PhysicalSize,
};

/// 完整的应用程序窗口
pub struct VizuaraWindow;

impl VizuaraWindow {
    /// 创建并运行窗口应用（一体化方法）
    pub async fn create_and_run() -> Result<()> {
        println!("🚀 开始创建 VizuaraWindow...");

        // 创建事件循环
        let event_loop = EventLoop::new().map_err(|e| {
            VizuaraError::RenderError(format!("Failed to create event loop: {}", e))
        })?;

        println!("✅ 事件循环创建成功");

        // 创建窗口
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Vizuara - 科学可视化库")
                .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
                .with_min_inner_size(winit::dpi::LogicalSize::new(400, 300))
                .build(&event_loop)
                .map_err(|e| {
                    VizuaraError::RenderError(format!("Failed to create window: {}", e))
                })?,
        );

        println!(
            "✅ 窗口创建成功: {}x{}",
            window.inner_size().width,
            window.inner_size().height
        );

        // 初始化渲染器和表面
        let size = window.inner_size();
        let (mut renderer, surface) = WgpuRenderer::new(&window, size).await?;

        println!("✅ 渲染器初始化成功");

        let window_id = window.id();
        let window_for_redraw = Arc::clone(&window);

        println!("🎮 开始主事件循环...");

        // 在事件循环中运行
        event_loop
            .run(move |event, control_flow| {
                match event {
                    Event::WindowEvent {
                        event,
                        window_id: event_window_id,
                        ..
                    } if event_window_id == window_id => {
                        match event {
                            WindowEvent::CloseRequested => {
                                println!("🔴 收到窗口关闭请求");
                                control_flow.exit();
                            }

                            WindowEvent::Resized(physical_size) => {
                                if physical_size.width > 0 && physical_size.height > 0 {
                                    println!(
                                        "📏 调整窗口大小: {}x{}",
                                        physical_size.width, physical_size.height
                                    );
                                    renderer.resize(physical_size, &surface);
                                }
                            }

                            WindowEvent::RedrawRequested => {
                                // 创建测试数据：三个不同颜色的点
                                let primitives = vec![Primitive::Points(vec![
                                    Point2::new(200.0, 200.0), // 左上
                                    Point2::new(600.0, 200.0), // 右上
                                    Point2::new(400.0, 500.0), // 底部中央
                                ])];

                                let styles = vec![Style::new()
                                    .fill_color(vizuara_core::Color::rgb(1.0, 0.2, 0.2))  // 红色
                                    .marker(vizuara_core::MarkerStyle::Circle, 10.0)];

                                match renderer.render(&surface, &primitives, &styles) {
                                    Ok(_) => {
                                        // 成功渲染
                                    }
                                    Err(e) => {
                                        eprintln!("❌ 渲染错误: {}", e);
                                    }
                                }
                            }

                            WindowEvent::KeyboardInput { event, .. } => {
                                println!("⌨️  键盘输入: {:?}", event);
                            }

                            WindowEvent::CursorMoved { .. } => {
                                // 可以在这里处理鼠标移动（但不打印，避免刷屏）
                            }

                            WindowEvent::MouseInput { state, button, .. } => {
                                println!("🖱️  鼠标点击: {:?} {:?}", button, state);
                            }

                            WindowEvent::MouseWheel { delta, .. } => {
                                println!("🎯 鼠标滚轮: {:?}", delta);
                            }

                            _ => {}
                        }
                    }

                    Event::AboutToWait => {
                        // 请求重绘
                        window_for_redraw.request_redraw();
                    }

                    Event::MemoryWarning => {
                        println!("⚠️  内存警告");
                    }

                    _ => {}
                }
            })
            .map_err(|e| VizuaraError::RenderError(format!("Event loop error: {}", e)))
    }
}
