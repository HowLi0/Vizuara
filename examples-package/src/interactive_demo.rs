use std::sync::Arc;
use std::time::{Duration, Instant};
use nalgebra::Point2;
use winit::event::{Event, WindowEvent, ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use vizuara_core::{Color, Style};
use vizuara_core::coords::LogicalPosition;
use vizuara_interactivity::viewport::Viewport;
use vizuara_interactivity::tools::{ToolManager, SimpleMouseEvent, SimpleKeyboardEvent};
use vizuara_plots::{PlotArea, scatter::ScatterPlot, line::LinePlot};
use vizuara_scene::{Scene, Figure};
use vizuara_wgpu::WgpuRenderer;

/// 交互功能演示：真实窗口 + 鼠标/键盘交互
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖱️  交互演示启动：Pan/Zoom/Select 即时生效");
    println!("提示：P 平移，Z 缩放，S 选择，R 重置，+/- 居中缩放，双击重置，ESC 退出");

    // 1) 数据
    let data_world: Vec<(f32, f32)> = create_demo_data()
        .into_iter()
        .map(|(x, y)| (x as f32, y as f32))
        .collect();

    // 2) 初始窗口/渲染器
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Vizuara - 交互演示")
            .with_inner_size(winit::dpi::LogicalSize::new(900u32, 680u32))
            .with_min_inner_size(winit::dpi::LogicalSize::new(480u32, 360u32))
            .build(&event_loop)?
    );
    let size = window.inner_size();
    let (mut renderer, surface) = WgpuRenderer::new(&window, size).await?;

    // 3) 初始化 Viewport + Tools（以数据范围为世界坐标）
    let (min_x, min_y, max_x, max_y) = data_bounds_f32(&data_world);
    let mut viewport = Viewport::from_data_range(
        size.width, size.height,
        (min_x as f64, min_y as f64),
        (max_x as f64, max_y as f64),
        0.1,
    );
    let default_bounds = viewport.bounds().clone();
    let mut tools = ToolManager::new();
    tools.set_default_viewport_bounds(default_bounds);

    // 鼠标位置/双击辅助状态
    let mut last_cursor = LogicalPosition { x: 0.0, y: 0.0 };
    let mut last_click: Option<(MouseButton, Instant)> = None;

    let window_id = window.id();
    let window_for_redraw = Arc::clone(&window);

    // 4) 事件循环
    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent { event, window_id: wid, .. } if wid == window_id => {
                    match event {
                        WindowEvent::CloseRequested => control_flow.exit(),

                        WindowEvent::Resized(physical_size) => {
                            if physical_size.width > 0 && physical_size.height > 0 {
                                renderer.resize(physical_size, &surface);
                                viewport.resize(physical_size.width, physical_size.height);
                                window_for_redraw.request_redraw();
                            }
                        }

                        WindowEvent::CursorMoved { position, .. } => {
                            last_cursor = LogicalPosition { x: position.x, y: position.y };
                            // 将移动交给当前工具（如平移进行中）
                            let _ = tools.handle_mouse_event(&SimpleMouseEvent::Move { position: last_cursor }, &mut viewport);
                        }

                        WindowEvent::MouseInput { state, button, .. } => {
                            match state {
                                ElementState::Pressed => {
                                    // 简易双击检测（300ms 内两次同键）
                                    let now = Instant::now();
                                    let is_double = last_click
                                        .as_ref()
                                        .map(|(b, t)| *b == button && now.duration_since(*t) <= Duration::from_millis(300))
                                        .unwrap_or(false);
                                    if is_double {
                                        let _ = tools.handle_mouse_event(&SimpleMouseEvent::DoubleClick { button, position: last_cursor }, &mut viewport);
                                        last_click = None;
                                    } else {
                                        last_click = Some((button, now));
                                        let _ = tools.handle_mouse_event(&SimpleMouseEvent::ButtonPress { button, position: last_cursor }, &mut viewport);
                                    }
                                    window_for_redraw.request_redraw();
                                }
                                ElementState::Released => {
                                    let _ = tools.handle_mouse_event(&SimpleMouseEvent::ButtonRelease { button, position: last_cursor }, &mut viewport);
                                    window_for_redraw.request_redraw();
                                }
                            }
                        }

                        WindowEvent::MouseWheel { delta, .. } => {
                            // 使用垂直滚动量作为缩放方向
                            let dy = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y as f64,
                                MouseScrollDelta::PixelDelta(pos) => pos.y / 60.0, // 近似换算（pos.y 已为 f64）
                            };
                            let _ = tools.handle_mouse_event(&SimpleMouseEvent::Scroll { delta: dy, position: last_cursor }, &mut viewport);
                            window_for_redraw.request_redraw();
                        }

                        WindowEvent::KeyboardInput { event, .. } => {
                            if event.state == ElementState::Pressed {
                                use winit::keyboard::KeyCode as KC;
                                let key_str = match event.physical_key {
                                    winit::keyboard::PhysicalKey::Code(code) => match code {
                                        KC::KeyP => Some("p"),
                                        KC::KeyZ => Some("z"),
                                        KC::KeyS => Some("s"),
                                        KC::KeyR => Some("r"),
                                        KC::Minus => Some("-"),
                                        KC::Equal => Some("+"), // 需要配合 Shift 才是 '+'，这里直接映射
                                        KC::Escape => Some("Escape"),
                                        _ => None,
                                    },
                                    _ => None,
                                };

                                if let Some(k) = key_str {
                                    let _ = tools.handle_keyboard_event(&SimpleKeyboardEvent::KeyPress { key: k.to_string() }, &mut viewport);
                                    // ESC 也交给工具，R 重置后也重绘
                                    window_for_redraw.request_redraw();
                                }
                            }
                        }

                        WindowEvent::RedrawRequested => {
                            // 依据当前 viewport 构建场景 -> 图元
                            let (prims, styles) = build_primitives_with_view(&data_world, &viewport);
                            if let Err(e) = renderer.render(&surface, &prims, &styles) {
                                eprintln!("渲染错误: {}", e);
                            }
                        }

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    window_for_redraw.request_redraw();
                }

                _ => {}
            }
        })
        .map_err(|e| e.into())
}

/// 根据当前 viewport 构建图元（包含轴、数据与选择框叠加）
fn build_primitives_with_view(data_world: &[(f32, f32)], viewport: &Viewport) -> (Vec<vizuara_core::Primitive>, Vec<Style>) {
    // 以 viewport 的世界边界作为当前显示范围
    let b = viewport.bounds();

    // 1) 使用当前世界边界设置比例尺
    let x_scale = vizuara_core::LinearScale::new(b.min_x as f32, b.max_x as f32);
    let y_scale = vizuara_core::LinearScale::new(b.min_y as f32, b.max_y as f32);

    // 2) 构建场景（PlotArea 使用像素坐标）
    let plot_area = PlotArea::new(80.0, 80.0, 700.0, 480.0);
    let scatter = ScatterPlot::new()
        .data(data_world)
        .x_scale(x_scale.clone())
        .y_scale(y_scale.clone())
        .size(6.0)
        .color(Color::rgb(0.2, 0.5, 0.9));

    let line = LinePlot::new()
        .data(data_world)
        .x_scale(x_scale)
        .y_scale(y_scale)
        .line_width(2.0)
        .color(Color::rgb(0.85, 0.3, 0.3));

    let scene = Scene::new(plot_area)
        .add_scatter_plot(scatter)
        .add_line_plot(line)
        .title("交互演示：Pan/Zoom/Select");

    let mut primitives = Figure::new(900.0, 680.0)
        .title("Vizuara 交互演示")
        .add_scene(scene)
        .generate_primitives();

    // 3) 若选择工具有矩形，叠加一个半透明矩形用于可视反馈
    // 注意：ToolManager 持有选择状态，但这里无法直接访问。
    // 我们采用从 viewport.zoom_level() 等生成的提示文本；
    // 选择框的可视化通常在工具管理器或全局状态中传入，这里留作后续扩展。

    // 简单 HUD 文本（显示缩放级别）
    primitives.push(vizuara_core::Primitive::Text {
        position: Point2::new(20.0, 30.0),
        content: format!("Zoom: {:.3}", viewport.zoom_level() as f32),
        size: 14.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        h_align: vizuara_core::HorizontalAlign::Left,
        v_align: vizuara_core::VerticalAlign::Bottom,
    });

    // 一个通用样式（应用于点/线/矩形等，无需一一匹配）
    let styles = vec![
        Style::new()
            .fill_color(Color::rgb(0.2, 0.5, 0.9))
            .stroke(Color::rgb(0.9, 0.3, 0.3), 2.0)
            .marker(vizuara_core::MarkerStyle::Circle, 8.0)
            .opacity(1.0),
    ];

    (primitives, styles)
}

fn create_demo_data() -> Vec<(f64, f64)> {
    let mut data = Vec::new();
    for i in 0..200 {
        let x = i as f64 * 0.05; // 更密集
        let y = 5.0 + 3.0 * (x * 0.7).sin() + 0.6 * (x * 2.1).cos();
        data.push((x, y));
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_demo_data_creation() {
        let data = create_demo_data();
        assert!(data.len() >= 100);
    }

    #[test]
    fn test_build_primitives_with_view() {
        let data: Vec<(f32,f32)> = create_demo_data().into_iter().map(|(x,y)|(x as f32,y as f32)).collect();
        let (min_x, min_y, max_x, max_y) = data_bounds_f32(&data);
        let mut vp = Viewport::from_data_range(800, 600, (min_x as f64, min_y as f64), (max_x as f64, max_y as f64), 0.1);
        let (prims, _styles) = build_primitives_with_view(&data, &vp);
        assert!(!prims.is_empty());
        vp.zoom_at_point(1.2, LogicalPosition{ x: 400.0, y: 300.0 }).unwrap();
        let (prims2, _styles2) = build_primitives_with_view(&data, &vp);
        assert!(!prims2.is_empty());
    }
}

fn data_bounds_f32(data: &[(f32, f32)]) -> (f32, f32, f32, f32) {
    let (mut min_x, mut min_y) = (f32::INFINITY, f32::INFINITY);
    let (mut max_x, mut max_y) = (f32::NEG_INFINITY, f32::NEG_INFINITY);
    for &(x, y) in data.iter() {
        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }
        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }
    }
    (min_x, min_y, max_x, max_y)
}
