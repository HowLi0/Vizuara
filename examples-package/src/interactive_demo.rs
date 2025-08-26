use vizuara_core::{
    primitive::Primitive,
    coords::{LogicalPosition, WorldPosition},
    Color,
};
use vizuara_plots::{scatter::ScatterPlot, line::LinePlot, PlotArea};
use vizuara_scene::{scene::Scene, figure::Figure};
use vizuara_interactivity::{
    viewport::{Viewport, ViewBounds},
    tools::{ToolManager, ToolType, SimpleMouseEvent, SimpleKeyboardEvent},
};
use winit::event::MouseButton;

/// 交互功能演示
pub fn main() {
    println!("=== Vizuara 交互功能演示 ===");
    
    // 创建测试数据
    let data_points = create_demo_data();
    println!("创建了 {} 个数据点", data_points.len());
    
    // 创建散点图 - 使用正确的API
    let data: Vec<(f32, f32)> = data_points.iter().map(|(x, y)| (*x as f32, *y as f32)).collect();
    let scatter_plot = ScatterPlot::new().data(&data);
    
    // 创建折线图 - 使用正确的API
    let line_plot = LinePlot::new()
        .data(&data)
        .color(Color::new(0.8, 0.2, 0.2, 1.0));
    
    // 创建场景
    let plot_area = PlotArea::new(50.0, 50.0, 700.0, 500.0);
    let scene = Scene::new(plot_area)
        .add_scatter_plot(scatter_plot)
        .add_line_plot(line_plot);
    
    // 创建图形
    let figure = Figure::new(800.0, 600.0).add_scene(scene);
    
    // 创建视口
    let mut viewport = Viewport::from_data_range(
        800, 600,
        (0.0, 0.0),
        (10.0, 10.0),
        0.1,
    );
    let bounds = viewport.bounds().clone();
    
    // 创建工具管理器
    let mut tool_manager = ToolManager::new();
    tool_manager.set_default_viewport_bounds(bounds);
    
    // 演示交互功能
    demonstrate_viewport_operations(&mut viewport);
    demonstrate_tool_operations(&mut tool_manager, &mut viewport);
    demonstrate_coordinate_transformations(&viewport);
    
    // 生成图元并显示统计
    let primitives = figure.generate_primitives();
    println!("\n=== 渲染统计 ===");
    println!("生成图元数量: {}", primitives.len());
    
    // 按类型统计图元
    let mut point_count = 0;
    let mut line_count = 0;
    let mut rect_count = 0;
    
    for primitive in &primitives {
        match primitive {
            Primitive::Point { .. } => point_count += 1,
            Primitive::Line { .. } => line_count += 1,
            Primitive::Rectangle { .. } => rect_count += 1,
            _ => {} // 处理其他类型的图元
        }
    }
    
    println!("点图元: {}", point_count);
    println!("线图元: {}", line_count);
    println!("矩形图元: {}", rect_count);
    
    println!("\n=== 演示完成 ===");
    println!("交互功能已成功实现！");
}

fn create_demo_data() -> Vec<(f64, f64)> {
    let mut data = Vec::new();
    
    // 创建正弦波数据
    for i in 0..50 {
        let x = i as f64 * 0.2;
        let y = 5.0 + 3.0 * (x * 0.5).sin() + 0.5 * (x * 2.0).cos();
        data.push((x, y));
    }
    
    data
}

fn demonstrate_viewport_operations(viewport: &mut Viewport) {
    println!("\n=== 视口操作演示 ===");
    
    println!("初始视口边界: {:?}", viewport.bounds());
    println!("初始视口大小: {:?}", viewport.size());
    
    // 演示坐标转换
    let screen_center = LogicalPosition { x: 400.0, y: 300.0 };
    let world_center = viewport.screen_to_world(screen_center);
    println!("屏幕中心 {:?} 对应世界坐标: {:?}", screen_center, world_center);
    
    // 演示缩放
    println!("\n缩放前视图范围宽度: {:.2}", viewport.bounds().width());
    viewport.zoom_at_point(2.0, screen_center).unwrap();
    println!("2倍缩放后视图范围宽度: {:.2}", viewport.bounds().width());
    
    // 演示平移
    let pan_delta = nalgebra::Vector2::new(50.0, 30.0);
    let old_center = viewport.bounds().center();
    viewport.pan(pan_delta).unwrap();
    let new_center = viewport.bounds().center();
    println!("平移前中心: ({:.2}, {:.2})", old_center.0, old_center.1);
    println!("平移后中心: ({:.2}, {:.2})", new_center.0, new_center.1);
    
    // 重置视图
    let reset_bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
    viewport.reset(reset_bounds);
    println!("重置后视图范围: {:?}", viewport.bounds());
}

fn demonstrate_tool_operations(tool_manager: &mut ToolManager, viewport: &mut Viewport) {
    println!("\n=== 工具操作演示 ===");
    
    println!("当前活动工具: {:?}", tool_manager.active_tool());
    
    // 演示工具切换
    tool_manager.activate_tool(ToolType::Zoom).unwrap();
    println!("切换到缩放工具: {:?}", tool_manager.active_tool());
    
    tool_manager.activate_tool(ToolType::Select).unwrap();
    println!("切换到选择工具: {:?}", tool_manager.active_tool());
    
    // 演示键盘快捷键
    let pan_key = SimpleKeyboardEvent::KeyPress { key: "p".to_string() };
    tool_manager.handle_keyboard_event(&pan_key, viewport).unwrap();
    println!("按 'p' 键切换到: {:?}", tool_manager.active_tool());
    
    // 演示鼠标交互
    let mouse_press = SimpleMouseEvent::ButtonPress {
        button: MouseButton::Left,
        position: LogicalPosition { x: 200.0, y: 150.0 },
    };
    
    let mouse_drag = SimpleMouseEvent::Move {
        position: LogicalPosition { x: 250.0, y: 200.0 },
    };
    
    let mouse_release = SimpleMouseEvent::ButtonRelease {
        button: MouseButton::Left,
        position: LogicalPosition { x: 250.0, y: 200.0 },
    };
    
    println!("模拟鼠标拖拽操作...");
    tool_manager.handle_mouse_event(&mouse_press, viewport).unwrap();
    tool_manager.handle_mouse_event(&mouse_drag, viewport).unwrap();
    tool_manager.handle_mouse_event(&mouse_release, viewport).unwrap();
    
    // 演示滚轮缩放
    tool_manager.activate_tool(ToolType::Zoom).unwrap();
    let scroll_event = SimpleMouseEvent::Scroll {
        delta: 1.0,
        position: LogicalPosition { x: 400.0, y: 300.0 },
    };
    
    let old_width = viewport.bounds().width();
    tool_manager.handle_mouse_event(&scroll_event, viewport).unwrap();
    let new_width = viewport.bounds().width();
    
    println!("滚轮缩放前宽度: {:.2}", old_width);
    println!("滚轮缩放后宽度: {:.2}", new_width);
}

fn demonstrate_coordinate_transformations(viewport: &Viewport) {
    println!("\n=== 坐标转换演示 ===");
    
    // 测试几个关键点的坐标转换
    let test_points = vec![
        LogicalPosition { x: 0.0, y: 0.0 },       // 左上角
        LogicalPosition { x: 400.0, y: 300.0 },   // 中心
        LogicalPosition { x: 800.0, y: 600.0 },   // 右下角
    ];
    
    for screen_point in test_points {
        let world_point = viewport.screen_to_world(screen_point);
        let back_to_screen = viewport.world_to_screen(world_point);
        
        println!("屏幕坐标 ({:.1}, {:.1}) -> 世界坐标 ({:.3}, {:.3}) -> 屏幕坐标 ({:.1}, {:.1})",
            screen_point.x, screen_point.y,
            world_point.x, world_point.y,
            back_to_screen.x, back_to_screen.y);
        
        // 验证往返转换的精度
        let error_x = (screen_point.x - back_to_screen.x).abs();
        let error_y = (screen_point.y - back_to_screen.y).abs();
        
        if error_x > 1e-10 || error_y > 1e-10 {
            println!("  警告：坐标转换误差较大 ({:.2e}, {:.2e})", error_x, error_y);
        }
    }
    
    // 演示世界坐标边界检查
    let world_points = vec![
        WorldPosition { x: 5.0, y: 5.0 },   // 视口内
        WorldPosition { x: -1.0, y: 5.0 },  // 视口外
        WorldPosition { x: 5.0, y: 12.0 },  // 视口外
    ];
    
    println!("\n世界坐标边界检查:");
    for world_point in world_points {
        let is_visible = viewport.contains_world_point(world_point);
        println!("点 ({:.1}, {:.1}) 是否在视口内: {}", 
            world_point.x, world_point.y, is_visible);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_demo_data_creation() {
        let data = create_demo_data();
        assert_eq!(data.len(), 50);
        
        // 验证数据范围
        for (x, y) in &data {
            assert!(*x >= 0.0 && *x <= 10.0);
            assert!(*y >= 0.0 && *y <= 10.0);
        }
    }

    #[test]
    fn test_interactive_demo_viewport_setup() {
        let viewport = Viewport::from_data_range(
            800, 600,
            (0.0, 0.0),
            (10.0, 10.0),
            0.1,
        );
        
        assert_eq!(viewport.size().x, 800);
        assert_eq!(viewport.size().y, 600);
        assert!(viewport.bounds().width() > 10.0); // 有边距
        assert!(viewport.bounds().height() > 10.0); // 有边距
    }

    #[test]
    fn test_interactive_demo_tool_management() {
        let mut tool_manager = ToolManager::new();
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        tool_manager.set_default_viewport_bounds(bounds.clone());
        
        assert_eq!(tool_manager.active_tool(), Some(ToolType::Pan));
        
        tool_manager.activate_tool(ToolType::Zoom).unwrap();
        assert_eq!(tool_manager.active_tool(), Some(ToolType::Zoom));
    }

    #[test]
    fn test_interactive_demo_complete_workflow() {
        // 创建数据
        let data = create_demo_data();
        assert!(!data.is_empty());
        
        // 创建图表
        let data: Vec<(f32, f32)> = data[..10].iter().map(|(x, y)| (*x as f32, *y as f32)).collect();
        let scatter_plot = ScatterPlot::new().data(&data);
        
        // 创建场景和图形
        let plot_area = PlotArea::new(50.0, 50.0, 700.0, 500.0);
        let scene = Scene::new(plot_area).add_scatter_plot(scatter_plot);
        let figure = Figure::new(800.0, 600.0).add_scene(scene);
        
        // 创建视口和工具
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        let mut viewport = Viewport::new(800, 600, bounds.clone());
        let mut tool_manager = ToolManager::new();
        
        // 测试交互
        let scroll_event = SimpleMouseEvent::Scroll {
            delta: 1.0,
            position: LogicalPosition { x: 400.0, y: 300.0 },
        };
        
        tool_manager.activate_tool(ToolType::Zoom).unwrap();
        let handled = tool_manager.handle_mouse_event(&scroll_event, &mut viewport).unwrap();
        assert!(handled);
        
        // 生成图元
        let primitives = figure.generate_primitives();
        assert!(!primitives.is_empty());
    }
}
