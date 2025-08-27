use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, DeviceEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    keyboard::{Key, NamedKey},
};
use std::sync::Arc;
use vizuara_core::{Result, VizuaraError, Color};
use vizuara_wgpu::{Wgpu3DRenderer, Vertex3D};
use vizuara_3d::{Scatter3D, Surface3D, Mesh3D};
use nalgebra::Point3;

/// 3D可视化窗口应用
pub struct Window3D {
    scatter_data: Option<Scatter3D>,
    surface_data: Option<Surface3D>,
    mesh_data: Option<Mesh3D>,
}

impl Window3D {
    /// 创建新的3D窗口
    pub fn new() -> Self {
        Self {
            scatter_data: None,
            surface_data: None,
            mesh_data: None,
        }
    }

    /// 添加3D散点图
    pub fn add_scatter3d(mut self, scatter: Scatter3D) -> Self {
        self.scatter_data = Some(scatter);
        self
    }

    /// 添加3D表面
    pub fn add_surface3d(mut self, surface: Surface3D) -> Self {
        self.surface_data = Some(surface);
        self
    }

    /// 添加3D网格
    pub fn add_mesh3d(mut self, mesh: Mesh3D) -> Self {
        self.mesh_data = Some(mesh);
        self
    }

    /// 运行3D窗口应用
    pub async fn run(self) -> Result<()> {
        println!("🌟 启动3D可视化窗口...");
        
        let event_loop = EventLoop::new()
            .map_err(|e| VizuaraError::RenderError(format!("Failed to create event loop: {}", e)))?;
        
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Vizuara 3D - 三维科学可视化")
                .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
                .with_min_inner_size(winit::dpi::LogicalSize::new(400, 300))
                .build(&event_loop)
                .map_err(|e| VizuaraError::RenderError(format!("Failed to create window: {}", e)))?
        );

        let size = window.inner_size();
        println!("✅ 3D窗口创建成功: {}x{}", size.width, size.height);

        // 初始化3D渲染器
    let (mut renderer, surface) = Wgpu3DRenderer::new(&window, size).await?;
        
        // 生成3D几何数据
        let (vertices, indices) = self.generate_3d_geometry();
        println!("📐 生成了 {} 个顶点，{} 个三角形", vertices.len(), indices.len() / 3);

        let mut mouse_pressed = false;
        let mut last_mouse_pos: Option<(f32, f32)> = None;
        let window_clone = window.clone();

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window_clone.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("🔚 关闭3D窗口");
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            println!("📏 窗口尺寸变更: {}x{}", physical_size.width, physical_size.height);
                            renderer.resize(*physical_size);
                            surface.configure(&renderer.device, &renderer.config);
                        }
                        WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                            mouse_pressed = *state == ElementState::Pressed;
                            if !mouse_pressed {
                                last_mouse_pos = None;
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            if mouse_pressed {
                                if let Some((last_x, last_y)) = last_mouse_pos {
                                    let delta_x = position.x as f32 - last_x;
                                    let delta_y = position.y as f32 - last_y;
                                    renderer.rotate_camera(delta_x, delta_y);
                                    window_clone.request_redraw();
                                }
                                last_mouse_pos = Some((position.x as f32, position.y as f32));
                            }
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let scroll_delta = match delta {
                                winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.01,
                            };
                            renderer.zoom_camera(-scroll_delta);
                            window_clone.request_redraw();
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            if event.state == ElementState::Pressed {
                                match event.logical_key {
                                    Key::Named(NamedKey::Escape) => {
                                        elwt.exit();
                                    }
                                    Key::Character(ref c) => {
                                        match c.as_str() {
                                            "r" | "R" => {
                                                // 重置相机
                                                renderer.set_camera(
                                                    Point3::new(4.0, 3.0, 2.0),
                                                    Point3::new(0.0, 0.0, 0.0),
                                                    nalgebra::Vector3::new(0.0, 1.0, 0.0)
                                                );
                                                window_clone.request_redraw();
                                                println!("📷 相机已重置");
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            match renderer.render_3d(&surface, &vertices, &indices) {
                                Ok(()) => {}
                                Err(e) => {
                                    eprintln!("渲染错误: {:?}", e);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { 
                    event: DeviceEvent::MouseMotion { delta }, 
                    .. 
                } => {
                    if mouse_pressed {
                        renderer.rotate_camera(delta.0 as f32, delta.1 as f32);
                        window_clone.request_redraw();
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

    /// 生成3D几何数据
    fn generate_3d_geometry(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // 添加散点图数据
        if let Some(scatter) = &self.scatter_data {
            let scatter_vertices = self.generate_scatter_vertices(scatter);
            let base_index = vertices.len() as u16;
            vertices.extend(scatter_vertices);
            
            // 为每个点生成小立方体
            for i in 0..scatter.point_count() {
                let base = base_index + (i * 8) as u16;
                // 立方体的12个三角形 (每个面2个三角形)
                let cube_indices = [
                    // 前面
                    base, base+1, base+2, base, base+2, base+3,
                    // 后面  
                    base+4, base+6, base+5, base+4, base+7, base+6,
                    // 左面
                    base, base+4, base+5, base, base+5, base+1,
                    // 右面
                    base+2, base+6, base+7, base+2, base+7, base+3,
                    // 上面
                    base+1, base+5, base+6, base+1, base+6, base+2,
                    // 下面
                    base, base+3, base+7, base, base+7, base+4,
                ];
                indices.extend_from_slice(&cube_indices);
            }
        }

        // 添加表面数据
        if let Some(surface) = &self.surface_data {
            let (surface_vertices, surface_indices) = self.generate_surface_geometry(surface);
            let base_index = vertices.len() as u16;
            vertices.extend(surface_vertices);
            indices.extend(surface_indices.iter().map(|&i| i + base_index));
        }

        // 添加网格数据
        if let Some(mesh) = &self.mesh_data {
            let (mesh_vertices, mesh_indices) = self.generate_mesh_geometry(mesh);
            let base_index = vertices.len() as u16;
            vertices.extend(mesh_vertices);
            indices.extend(mesh_indices.iter().map(|&i| i + base_index));
        }

        // 如果没有任何数据，生成一个简单的测试立方体
        if vertices.is_empty() {
            self.generate_test_cube()
        } else {
            (vertices, indices)
        }
    }

    /// 生成散点图顶点
    fn generate_scatter_vertices(&self, scatter: &Scatter3D) -> Vec<Vertex3D> {
        let mut vertices = Vec::new();
        let size = 0.05; // 立方体大小

        for i in 0..scatter.point_count() {
            if let Some(point) = scatter.point_at(i) {
                let color = scatter.color_at(i).unwrap_or(Color::rgb(1.0, 0.0, 0.0));
                let color_array = [color.r, color.g, color.b, color.a];

                // 生成小立方体的8个顶点
                let positions = [
                    [point.x - size, point.y - size, point.z - size],
                    [point.x + size, point.y - size, point.z - size],
                    [point.x + size, point.y + size, point.z - size],
                    [point.x - size, point.y + size, point.z - size],
                    [point.x - size, point.y - size, point.z + size],
                    [point.x + size, point.y - size, point.z + size],
                    [point.x + size, point.y + size, point.z + size],
                    [point.x - size, point.y + size, point.z + size],
                ];

                for pos in positions {
                    vertices.push(Vertex3D::new(pos, color_array));
                }
            }
        }

        vertices
    }

    /// 生成表面几何数据
    fn generate_surface_geometry(&self, surface: &Surface3D) -> (Vec<Vertex3D>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mesh = surface.mesh();
        let width = mesh.width;
        let height = mesh.height;

        // 生成顶点
        for y in 0..height {
            for x in 0..width {
                if let Some(point) = mesh.point_at(x, y) {
                    let color = [0.3, 0.7, 1.0, 1.0]; // 蓝色表面
                    vertices.push(Vertex3D::new([point.x, point.y, point.z], color));
                }
            }
        }

        // 生成三角形索引
        for y in 0..height-1 {
            for x in 0..width-1 {
                let i0 = (y * width + x) as u16;
                let i1 = (y * width + x + 1) as u16;
                let i2 = ((y + 1) * width + x) as u16;
                let i3 = ((y + 1) * width + x + 1) as u16;

                // 两个三角形组成一个四边形
                indices.extend_from_slice(&[i0, i1, i2, i1, i3, i2]);
            }
        }

        (vertices, indices)
    }

    /// 生成网格几何数据
    fn generate_mesh_geometry(&self, mesh: &Mesh3D) -> (Vec<Vertex3D>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..mesh.vertex_count() {
            if let Some(vertex) = mesh.vertex_at(i) {
                let color = [0.8, 0.4, 0.1, 1.0]; // 橙色网格
                vertices.push(Vertex3D::new([vertex.x, vertex.y, vertex.z], color));
            }
        }

        for i in 0..mesh.triangle_count() {
            if let Some(triangle) = mesh.triangle_at(i) {
                indices.extend_from_slice(&[triangle.0 as u16, triangle.1 as u16, triangle.2 as u16]);
            }
        }

        (vertices, indices)
    }

    /// 生成测试立方体
    fn generate_test_cube(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        let vertices = vec![
            // 前面 (红色)
            Vertex3D::new([-1.0, -1.0,  1.0], [1.0, 0.0, 0.0, 1.0]),
            Vertex3D::new([ 1.0, -1.0,  1.0], [1.0, 0.0, 0.0, 1.0]),
            Vertex3D::new([ 1.0,  1.0,  1.0], [1.0, 0.0, 0.0, 1.0]),
            Vertex3D::new([-1.0,  1.0,  1.0], [1.0, 0.0, 0.0, 1.0]),
            // 后面 (绿色)
            Vertex3D::new([-1.0, -1.0, -1.0], [0.0, 1.0, 0.0, 1.0]),
            Vertex3D::new([ 1.0, -1.0, -1.0], [0.0, 1.0, 0.0, 1.0]),
            Vertex3D::new([ 1.0,  1.0, -1.0], [0.0, 1.0, 0.0, 1.0]),
            Vertex3D::new([-1.0,  1.0, -1.0], [0.0, 1.0, 0.0, 1.0]),
        ];

        let indices = vec![
            // 前面
            0, 1, 2, 0, 2, 3,
            // 后面
            4, 6, 5, 4, 7, 6,
            // 左面
            0, 4, 5, 0, 5, 1,
            // 右面
            2, 6, 7, 2, 7, 3,
            // 上面
            1, 5, 6, 1, 6, 2,
            // 下面
            0, 3, 7, 0, 7, 4,
        ];

        (vertices, indices)
    }
}

impl Default for Window3D {
    fn default() -> Self {
        Self::new()
    }
}
