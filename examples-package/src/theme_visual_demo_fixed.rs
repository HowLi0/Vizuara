use std::sync::Arc;
use nalgebra::Point2;
use winit::event::{Event, WindowEvent, ElementState, KeyEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::keyboard::{KeyCode, PhysicalKey};

use vizuara_core::{Color, Primitive, Style, HorizontalAlign, VerticalAlign};
use vizuara_wgpu::WgpuRenderer;
use vizuara_themes::{ThemeManager, ThemePresets};

struct ThemeDemo {
    current_theme_index: usize,
    themes: Vec<(&'static str, fn() -> vizuara_themes::Theme)>,
}

impl ThemeDemo {
    fn new() -> Self {
        let themes = vec![
            ("Default", ThemePresets::default as fn() -> vizuara_themes::Theme),
            ("Dark", ThemePresets::dark as fn() -> vizuara_themes::Theme),
            ("Scientific", ThemePresets::scientific as fn() -> vizuara_themes::Theme),
            ("Nature", ThemePresets::nature as fn() -> vizuara_themes::Theme),
            ("Warm", ThemePresets::warm as fn() -> vizuara_themes::Theme),
            ("Cool", ThemePresets::cool as fn() -> vizuara_themes::Theme),
            ("HighContrast", ThemePresets::high_contrast as fn() -> vizuara_themes::Theme),
            ("Minimal", ThemePresets::minimal as fn() -> vizuara_themes::Theme),
            ("Professional", ThemePresets::professional as fn() -> vizuara_themes::Theme),
        ];
        
        Self {
            current_theme_index: 0,
            themes,
        }
    }

    fn next_theme(&mut self) {
        self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
        let theme_fn = self.themes[self.current_theme_index].1;
        let new_theme = theme_fn();
        
        let manager = ThemeManager::instance();
        let _ = manager.set_theme(new_theme);
    }

    fn previous_theme(&mut self) {
        if self.current_theme_index == 0 {
            self.current_theme_index = self.themes.len() - 1;
        } else {
            self.current_theme_index -= 1;
        }
        let theme_fn = self.themes[self.current_theme_index].1;
        let new_theme = theme_fn();
        
        let manager = ThemeManager::instance();
        let _ = manager.set_theme(new_theme);
    }

    fn handle_input(&mut self, event: &KeyEvent) -> bool {
        if event.state == ElementState::Pressed {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::Space) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                    self.next_theme();
                    true
                }
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    self.previous_theme();
                    true
                }
                PhysicalKey::Code(KeyCode::Escape) => false,
                _ => true,
            }
        } else {
            true
        }
    }

    fn create_primitives(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        
        let manager = ThemeManager::instance();
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
        
        // 散点图 - 使用基础圆形，颜色通过样式指定
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

        // 柱状图 - 使用带样式的矩形
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
                stroke: Some((primary_color, 1.0)),
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
        let theme_name = self.themes[self.current_theme_index].0;

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

        primitives
    }

    fn create_styles(&self) -> Vec<Style> {
        let manager = ThemeManager::instance();
        let current_theme = manager.current_theme();
        
        let primary_color = current_theme.primary.primary;
        let secondary_color = current_theme.primary.secondary;
        let text_color = current_theme.get_text_color();
        
        let mut styles = vec![Style::default()]; // 背景
        
        // 散点图样式 (5个点)
        for _ in 0..5 {
            styles.push(Style {
                stroke_color: Some(primary_color),
                stroke_width: 2.0,
                opacity: 1.0,
                ..Default::default()
            });
        }
        
        // 折线图样式 (4条线)
        for _ in 0..4 {
            styles.push(Style {
                stroke_color: Some(secondary_color),
                stroke_width: 3.0,
                opacity: 1.0,
                ..Default::default()
            });
        }
        
        // 柱状图样式 (5个柱子，已在 RectangleStyled 中指定颜色)
        for _ in 0..5 {
            styles.push(Style::default());
        }
        
        // 坐标轴样式 (2条轴)
        for _ in 0..2 {
            styles.push(Style {
                stroke_color: Some(text_color),
                stroke_width: 2.0,
                opacity: 0.8,
                ..Default::default()
            });
        }
        
        // 文本样式 (2个文本)
        for _ in 0..2 {
            styles.push(Style::default());
        }
        
        styles
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Vizuara 主题演示");
    println!("控制键：");
    println!("  空格/→ - 下一个主题");
    println!("  ← - 上一个主题");
    println!("  ESC - 退出");

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Vizuara 主题演示")
            .with_inner_size(winit::dpi::LogicalSize::new(1000, 700))
            .build(&event_loop)?
    );

    let size = window.inner_size();
    let (mut renderer, surface) = WgpuRenderer::new(&window, size).await?;
    let mut demo = ThemeDemo::new();

    // 设置初始主题
    let manager = ThemeManager::instance();
    let _ = manager.set_theme(ThemePresets::default());

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    if !demo.handle_input(event) {
                        elwt.exit();
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        renderer.resize(*physical_size, &surface);
                    }
                }
                WindowEvent::RedrawRequested => {
                    let primitives = demo.create_primitives();
                    let styles = demo.create_styles();
                    
                    if let Err(e) = renderer.render(&surface, &primitives, &styles) {
                        eprintln!("渲染错误: {:?}", e);
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
