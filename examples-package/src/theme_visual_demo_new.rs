use winit::{
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::sync::Arc;
use pollster;
use vizuara_wgpu::WgpuRenderer;
use vizuara_core::{Color, Primitive, Style, HorizontalAlign, VerticalAlign};
use vizuara_themes::{ThemeManager, ThemePresets, ThemeBuilder};
use nalgebra::Point2;

struct ThemeDemo {
    current_theme_index: usize,
    themes: Vec<String>,
}

impl ThemeDemo {
    fn new() -> Self {
        let themes = vec![
            "Default".to_string(),
            "Light".to_string(),
            "Dark".to_string(),
            "Blue".to_string(),
            "Green".to_string(),
            "Warm".to_string(),
            "Cool".to_string(),
            "HighContrast".to_string(),
            "Professional".to_string(),
        ];
        
        Self {
            current_theme_index: 0,
            themes,
        }
    }

    fn next_theme(&mut self) {
        self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
        let theme_name = &self.themes[self.current_theme_index];
        let preset = match theme_name.as_str() {
            "Default" => ThemePresets::default(),
            "Light" => ThemePresets::light(),
            "Dark" => ThemePresets::dark(),
            "Blue" => ThemePresets::blue(),
            "Green" => ThemePresets::green(),
            "Warm" => ThemePresets::warm(),
            "Cool" => ThemePresets::cool(),
            "HighContrast" => ThemePresets::high_contrast(),
            "Professional" => ThemePresets::professional(),
            _ => ThemePresets::default(),
        };
        
        if let Ok(mut manager) = ThemeManager::global() {
            let _ = manager.set_theme(preset);
        }
    }

    fn previous_theme(&mut self) {
        if self.current_theme_index == 0 {
            self.current_theme_index = self.themes.len() - 1;
        } else {
            self.current_theme_index -= 1;
        }
        let theme_name = &self.themes[self.current_theme_index];
        let preset = match theme_name.as_str() {
            "Default" => ThemePresets::default(),
            "Light" => ThemePresets::light(),
            "Dark" => ThemePresets::dark(),
            "Blue" => ThemePresets::blue(),
            "Green" => ThemePresets::green(),
            "Warm" => ThemePresets::warm(),
            "Cool" => ThemePresets::cool(),
            "HighContrast" => ThemePresets::high_contrast(),
            "Professional" => ThemePresets::professional(),
            _ => ThemePresets::default(),
        };
        
        if let Ok(mut manager) = ThemeManager::global() {
            let _ = manager.set_theme(preset);
        }
    }

    fn handle_input(&mut self, input: KeyboardInput) -> bool {
        if let Some(keycode) = input.virtual_keycode {
            if input.state == ElementState::Pressed {
                match keycode {
                    VirtualKeyCode::Space | VirtualKeyCode::Right => {
                        self.next_theme();
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.previous_theme();
                        true
                    }
                    VirtualKeyCode::Escape => false,
                    _ => true,
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    fn create_primitives(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        
        if let Ok(manager) = ThemeManager::global() {
            let current_theme = manager.current_theme();
            
            // 获取当前主题的颜色
            let bg_color = current_theme.primary.background;
            let primary_color = current_theme.primary.primary;
            let secondary_color = current_theme.primary.secondary;
            let accent_color = current_theme.primary.accent;
            let text_color = current_theme.get_text_color();

            // 背景矩形
            primitives.push(Primitive::RectangleStyled {
                min: Point2::new(0.0, 0.0),
                max: Point2::new(1000.0, 700.0),
                fill: bg_color,
                stroke: None,
            });

            // 创建一些示例图形来展示主题
            
            // 散点图
            let scatter_points = vec![
                Point2::new(100.0, 150.0),
                Point2::new(150.0, 200.0),
                Point2::new(200.0, 175.0),
                Point2::new(250.0, 225.0),
                Point2::new(300.0, 190.0),
            ];
            
            for point in &scatter_points {
                primitives.push(Primitive::Circle {
                    center: *point,
                    radius: 8.0,
                });
            }

            // 折线图
            let line_points = vec![
                Point2::new(400.0, 200.0),
                Point2::new(450.0, 150.0),
                Point2::new(500.0, 180.0),
                Point2::new(550.0, 120.0),
                Point2::new(600.0, 160.0),
            ];
            
            for i in 0..line_points.len() - 1 {
                primitives.push(Primitive::Line {
                    start: line_points[i],
                    end: line_points[i + 1],
                });
            }

            // 柱状图
            let bar_data = vec![0.8, 0.6, 1.0, 0.4, 0.7];
            let bar_width = 40.0;
            let bar_spacing = 60.0;
            let bar_base_y = 450.0;
            let bar_max_height = 100.0;
            
            for (i, &value) in bar_data.iter().enumerate() {
                let x = 100.0 + i as f32 * bar_spacing;
                let height = value * bar_max_height;
                let y = bar_base_y - height;
                
                primitives.push(Primitive::RectangleStyled {
                    min: Point2::new(x, y),
                    max: Point2::new(x + bar_width, bar_base_y),
                    fill: accent_color,
                    stroke: None,
                });
            }

            // 坐标轴
            // X轴
            primitives.push(Primitive::Line {
                start: Point2::new(80.0, bar_base_y),
                end: Point2::new(420.0, bar_base_y),
            });
            
            // Y轴
            primitives.push(Primitive::Line {
                start: Point2::new(80.0, 320.0),
                end: Point2::new(80.0, bar_base_y),
            });

            // 添加文本标签
            let theme_name = &self.themes[self.current_theme_index];

            primitives.push(Primitive::Text {
                position: Point2::new(20.0, 30.0),
                content: format!("主题: {} ({}/{})", 
                    theme_name, 
                    self.current_theme_index + 1, 
                    self.themes.len()),
                size: 24.0,
                color: text_color,
                h_align: HorizontalAlign::Left,
                v_align: VerticalAlign::Top,
            });

            primitives.push(Primitive::Text {
                position: Point2::new(20.0, 650.0),
                content: "按 空格/→ 切换主题, ← 返回上一主题, ESC 退出".to_string(),
                size: 16.0,
                color: text_color,
                h_align: HorizontalAlign::Left,
                v_align: VerticalAlign::Top,
            });
        }

        primitives
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vizuara 主题演示")
        .with_inner_size(winit::dpi::LogicalSize::new(1000, 700))
        .build(&event_loop)?;

    let window = Arc::new(window);
    let size = window.inner_size();
    
    let (mut renderer, surface) = pollster::block_on(WgpuRenderer::new(&window, size))?;
    let mut demo = ThemeDemo::new();

    // 设置初始主题
    if let Ok(mut manager) = ThemeManager::global() {
        let _ = manager.set_theme(ThemePresets::default());
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if !demo.handle_input(*input) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        renderer.resize(*physical_size, &surface);
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let primitives = demo.create_primitives();
                let styles = vec![Style::default(); primitives.len()];
                
                if let Err(e) = renderer.render(&surface, &primitives, &styles) {
                    eprintln!("渲染错误: {:?}", e);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
