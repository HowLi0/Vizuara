//! Figure 专用窗口
//! 
//! 提供直接渲染 Figure 对象的窗口应用

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;
use vizuara_core::{Result, VizuaraError, Style};
use vizuara_wgpu::WgpuRenderer;
use vizuara_scene::Figure;

/// 专门用于渲染 Figure 的窗口应用
pub struct FigureWindow {
    figure: Figure,
}

impl FigureWindow {
    /// 创建新的 Figure 窗口
    pub fn new(figure: Figure) -> Self {
        Self { figure }
    }

    /// 运行窗口应用，显示 Figure
    pub async fn run(self) -> Result<()> {
        println!("🖼️  启动 Figure 窗口渲染...");
        
        // 创建事件循环
        let event_loop = EventLoop::new()
            .map_err(|e| VizuaraError::RenderError(format!("Failed to create event loop: {}", e)))?;
        
        // 获取 Figure 尺寸
        let (fig_width, fig_height) = self.figure.size();
        
        // 创建窗口
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Vizuara - 科学可视化")
                .with_inner_size(winit::dpi::LogicalSize::new(fig_width as u32, fig_height as u32))
                .with_min_inner_size(winit::dpi::LogicalSize::new(400, 300))
                .build(&event_loop)
                .map_err(|e| VizuaraError::RenderError(format!("Failed to create window: {}", e)))?
        );
        
        println!("✅ 窗口创建成功: {}x{}", window.inner_size().width, window.inner_size().height);
        
        // 初始化渲染器
        let size = window.inner_size();
        let (mut renderer, surface) = WgpuRenderer::new(&*window, size).await?;
        
        println!("✅ 渲染器初始化成功");
        
        // 生成 Figure 的渲染图元
        let primitives = self.figure.generate_primitives();
        println!("📊 生成了 {} 个渲染图元", primitives.len());
        
        // 创建统一的样式（后续可以从 Figure 中获取）
        let styles = vec![
            Style::new()
                .fill_color(vizuara_core::Color::rgb(0.2, 0.4, 0.8))
                .stroke(vizuara_core::Color::rgb(0.1, 0.1, 0.1), 1.0)
        ];
        
        let window_id = window.id();
        let window_for_redraw = Arc::clone(&window);
        
        println!("🎮 开始渲染循环...");
        
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
                            println!("🔴 收到窗口关闭请求");
                            control_flow.exit();
                        }
                        
                        WindowEvent::Resized(physical_size) => {
                            if physical_size.width > 0 && physical_size.height > 0 {
                                println!("📏 调整窗口大小: {}x{}", physical_size.width, physical_size.height);
                                renderer.resize(physical_size, &surface);
                                window_for_redraw.request_redraw();
                            }
                        }
                        
                        WindowEvent::RedrawRequested => {
                            // 渲染 Figure 的图元
                            match renderer.render(&surface, &primitives, &styles) {
                                Ok(_) => {
                                    // 渲染成功
                                }
                                Err(e) => {
                                    eprintln!("❌ 渲染错误: {}", e);
                                }
                            }
                        }
                        
                        WindowEvent::KeyboardInput { event, .. } => {
                            if event.state == winit::event::ElementState::Pressed {
                                match event.physical_key {
                                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => {
                                        println!("🔑 ESC 键退出");
                                        control_flow.exit();
                                    }
                                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyR) => {
                                        println!("🔄 R 键刷新");
                                        window_for_redraw.request_redraw();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        
                        _ => {}
                    }
                }
                
                Event::AboutToWait => {
                    // 请求重绘以保持动画流畅
                    window_for_redraw.request_redraw();
                }
                
                _ => {}
            }
        })
        .map_err(|e| VizuaraError::RenderError(format!("Event loop error: {}", e)))
    }
}

/// 便捷方法：直接显示 Figure
pub async fn show_figure(figure: Figure) -> Result<()> {
    let window = FigureWindow::new(figure);
    window.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizuara_scene::Scene;
    use vizuara_plots::{ScatterPlot, PlotArea};
    use vizuara_core::Color;

    #[test]
    fn test_figure_window_creation() {
        let figure = Figure::new(800.0, 600.0);
        let window = FigureWindow::new(figure);
        
        // 基础创建测试
        assert_eq!(window.figure.size(), (800.0, 600.0));
    }

    #[test]
    fn test_figure_with_scatter_plot() {
        // 创建测试数据
        let data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 1.0)];
        
        // 创建散点图
        let scatter = ScatterPlot::new()
            .data(&data)
            .color(Color::rgb(1.0, 0.0, 0.0))
            .auto_scale();
        
        // 创建场景
        let plot_area = PlotArea::new(100.0, 100.0, 600.0, 400.0);
        let scene = Scene::new(plot_area)
            .add_scatter_plot(scatter);
        
        // 创建图形
        let figure = Figure::new(800.0, 600.0)
            .title("Test Scatter Plot")
            .add_scene(scene);
        
        let window = FigureWindow::new(figure);
        
        // 验证能生成图元
        let primitives = window.figure.generate_primitives();
        assert!(!primitives.is_empty());
    }
}
