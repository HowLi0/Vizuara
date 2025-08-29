use bytemuck::{Pod, Zeroable};
use vizuara_core::{Color, HorizontalAlign, Primitive, Result, Style, VerticalAlign, VizuaraError};
use wgpu::util::DeviceExt;
use winit::window::Window;
//use nalgebra::Point2;
use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Wrap,
};
use std::collections::HashMap;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    fn new(position: [f32; 2], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

/// WGPU 渲染器
pub struct WgpuRenderer {
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    // 文本渲染
    font_system: FontSystem,
    swash_cache: SwashCache,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer,
    // 文本缓存：key=(content,size,h_align,v_align)，值=(Buffer, color)
    text_cache: HashMap<(String, u32, u8, u8), Buffer>,
}

impl WgpuRenderer {
    /// 创建新的渲染器
    pub async fn new(
        window: &Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Result<(Self, wgpu::Surface<'_>)> {
        // 尝试不同后端以适配更多环境（优先 GL，再尝试 Vulkan）
        let backend_candidates = [
            wgpu::Backends::GL,
            wgpu::Backends::VULKAN,
            wgpu::Backends::PRIMARY,
        ];

        let mut last_err: Option<String> = None;
        for backends in backend_candidates {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends,
                ..Default::default()
            });

            // 创建表面
            let surface = match instance.create_surface(window) {
                Ok(s) => s,
                Err(e) => {
                    last_err = Some(format!("create_surface failed for {:?}: {}", backends, e));
                    continue;
                }
            };

            // 选择适配器
            let Some(adapter) = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
            else {
                last_err = Some(format!("request_adapter returned None for {:?}", backends));
                continue;
            };

            let surface_caps = surface.get_capabilities(&adapter);
            if surface_caps.formats.is_empty() {
                last_err = Some(format!(
                    "No supported surface formats for backend {:?}. This environment may not support presenting (WSL/remote/llvmpipe).",
                    backends
                ));
                continue;
            }

            // 选择合适的格式 & 模式
            let surface_format = surface_caps
                .formats
                .iter()
                .copied()
                .find(|f| f.is_srgb())
                .unwrap_or(surface_caps.formats[0]);

            let present_mode = if surface_caps
                .present_modes
                .contains(&wgpu::PresentMode::Mailbox)
            {
                wgpu::PresentMode::Mailbox
            } else if surface_caps
                .present_modes
                .contains(&wgpu::PresentMode::Immediate)
            {
                wgpu::PresentMode::Immediate
            } else {
                wgpu::PresentMode::Fifo
            };

            let alpha_mode = surface_caps
                .alpha_modes
                .first()
                .copied()
                .unwrap_or(wgpu::CompositeAlphaMode::Auto);

            // 创建设备
            let (device, queue) = match adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                        label: None,
                    },
                    None,
                )
                .await
            {
                Ok(dq) => dq,
                Err(e) => {
                    last_err = Some(format!("request_device failed for {:?}: {}", backends, e));
                    continue;
                }
            };

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode,
                alpha_mode,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            // 配置表面（能力检查已做，正常情况下不应 panic）
            surface.configure(&device, &config);

            // 创建渲染管线
            let render_pipeline = Self::create_render_pipeline(&device, &config)?;

            // 初始化文本渲染
            let mut font_system = FontSystem::new();
            // 尝试加载常见字体（增强中英文显示一致性），失败则忽略
            {
                let db = font_system.db_mut();
                let font_candidates = [
                    "/usr/share/fonts/truetype/noto/NotoSansSC-Regular.ttf",
                    "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
                    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                    "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
                    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                ];
                for path in font_candidates {
                    let _ = db.load_font_file(path);
                }
            }
            let swash_cache = SwashCache::new();
            let mut text_atlas = TextAtlas::new(&device, &queue, config.format);
            let text_renderer = TextRenderer::new(
                &mut text_atlas,
                &device,
                wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                None,
            );

            let renderer = WgpuRenderer {
                _instance: instance,
                _adapter: adapter,
                device,
                queue,
                config,
                size,
                render_pipeline,
                font_system,
                swash_cache,
                text_atlas,
                text_renderer,
                text_cache: HashMap::new(),
            };

            return Ok((renderer, surface));
        }

        Err(VizuaraError::RenderError(format!(
            "Failed to initialize GPU surface. {}\n- Try installing/updating Vulkan/GL drivers.\n- On WSL, enable WSLg GPU acceleration.\n- Or run with WGPU_BACKEND=gl",
            last_err.unwrap_or_else(|| "Unknown error".to_string())
        )))
    }

    /// 创建渲染管线
    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Result<wgpu::RenderPipeline> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/basic.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // 关闭背面剔除，避免不同图元缠绕方向不一致导致的消隐
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(render_pipeline)
    }

    /// 获取底层设备（用于与外部渲染器如 egui 共享）
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// 获取底层队列
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// 获取当前表面格式
    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    /// 重新配置表面（例如在 SurfaceError::Lost/Outdated 时调用）
    pub fn reconfigure(&self, surface: &wgpu::Surface) {
        surface.configure(&self.device, &self.config);
    }

    /// 调整窗口大小
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface: &wgpu::Surface) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            surface.configure(&self.device, &self.config);
            // 缓存与视口相关，尺寸改变后清空缓存以重建
            self.text_cache.clear();
        }
    }

    /// 在给定的视图上渲染（不获取/呈现交换链）。
    /// 典型用法：你的外部代码先获取 `SurfaceTexture` 和 `TextureView`，
    /// 使用该方法完成 Vizuara 的绘制，然后在同一帧上叠加 egui。
    pub fn render_to_view(
        &mut self,
        view: &wgpu::TextureView,
        primitives: &[Primitive],
        styles: &[Style],
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<()> {
        // 转换图元为顶点，同时收集文本
        let mut texts: Vec<(String, f32, f32, f32, Color, HorizontalAlign, VerticalAlign)> =
            Vec::new();
        let vertices = self.primitives_to_vertices_collect_text(primitives, styles, &mut texts);

        if !vertices.is_empty() {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.2,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.draw(0..vertices.len() as u32, 0..1);
            }

            // 文本 pass：在已清屏并绘制图形后，加载颜色叠加文本
            self.draw_texts(encoder, view, &mut texts)?;
        } else {
            // 即使没有顶点也要清屏
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        Ok(())
    }

    /// 渲染帧
    pub fn render(
        &mut self,
        surface: &wgpu::Surface,
        primitives: &[Primitive],
        styles: &[Style],
    ) -> Result<()> {
        let output = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                // 表面丢失或过时，重新配置
                surface.configure(&self.device, &self.config);
                return Ok(()); // 这一帧跳过渲染
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                return Err(VizuaraError::RenderError("GPU out of memory".to_string()));
            }
            Err(e) => {
                return Err(VizuaraError::RenderError(format!(
                    "Failed to get surface texture: {}",
                    e
                )));
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // 复用通用路径在视图上绘制
        self.render_to_view(&view, primitives, styles, &mut encoder)?;

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// 绘制文本：使用 glyphon
    fn draw_texts(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        texts: &mut [(String, f32, f32, f32, Color, HorizontalAlign, VerticalAlign)],
    ) -> Result<()> {
        if texts.is_empty() {
            return Ok(());
        }

        // 第一阶段：确保缓存存在（只做插入，不持有引用，避免与后续不可变借用冲突）
        let mut keys: Vec<(String, u32, u8, u8)> = Vec::with_capacity(texts.len());
        for (content, _x, _y, size, _color, h, v) in texts.iter() {
            let h_code = match h {
                HorizontalAlign::Left => 0u8,
                HorizontalAlign::Center => 1u8,
                HorizontalAlign::Right => 2u8,
            };
            let v_code = match v {
                VerticalAlign::Top => 0u8,
                VerticalAlign::Middle => 1u8,
                VerticalAlign::Baseline => 2u8,
                VerticalAlign::Bottom => 3u8,
            };
            let key = (content.clone(), (*size as u32), h_code, v_code);
            if !self.text_cache.contains_key(&key) {
                let mut buf = Buffer::new(&mut self.font_system, Metrics::new(*size, *size));
                buf.set_size(
                    &mut self.font_system,
                    self.size.width as f32,
                    self.size.height as f32,
                );
                buf.set_text(
                    &mut self.font_system,
                    content,
                    Attrs::new().family(Family::SansSerif),
                    Shaping::Advanced,
                );
                buf.set_wrap(&mut self.font_system, Wrap::None);
                self.text_cache.insert(key.clone(), buf);
            }
            keys.push(key);
        }

        // 构造文本区域
        let to_u8 = |v: f32| -> u8 { (v.clamp(0.0, 1.0) * 255.0).round() as u8 };
        let mut areas: Vec<TextArea> = Vec::new();
        for ((content, x, y, size, color, h, v), key) in texts.iter().zip(keys.iter()) {
            let buf = self
                .text_cache
                .get(key)
                .expect("text buffer must exist after first pass");
            // 简单锚点偏移：按字号估算 em 高度，左中右/上中下
            let em = *size; // 以 size 作为高度估计
            let avg_w = if !content.is_ascii() {
                *size * 0.9
            } else {
                *size * 0.6
            };
            let width_est = content.chars().count() as f32 * avg_w;
            let mut left = *x;
            let mut top = *y;
            // 水平
            match h {
                HorizontalAlign::Left => {}
                HorizontalAlign::Center => {
                    left -= width_est / 2.0;
                }
                HorizontalAlign::Right => {
                    left -= width_est;
                }
            }
            // 垂直（top 为文本行框的上边）
            match v {
                VerticalAlign::Top => {}
                VerticalAlign::Middle => {
                    top -= em / 2.0;
                }
                VerticalAlign::Baseline => {
                    top -= em * 0.8;
                }
                VerticalAlign::Bottom => {
                    top -= em;
                }
            }
            areas.push(TextArea {
                buffer: buf,
                left,
                top,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: self.config.width as i32,
                    bottom: self.config.height as i32,
                },
                default_color: glyphon::Color::rgba(
                    to_u8(color.r),
                    to_u8(color.g),
                    to_u8(color.b),
                    to_u8(color.a),
                ),
            });
        }

        // 准备文本
        if let Err(e) = self.text_renderer.prepare(
            &self.device,
            &self.queue,
            &mut self.font_system,
            &mut self.text_atlas,
            Resolution {
                width: self.config.width,
                height: self.config.height,
            },
            areas,
            &mut self.swash_cache,
        ) {
            return Err(VizuaraError::RenderError(format!(
                "Text prepare failed: {}",
                e
            )));
        }

        // 开启一个加载现有颜色的 pass，以便在图形上方绘制文字
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Text Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            if let Err(e) = self
                .text_renderer
                .render(&self.text_atlas, &mut render_pass)
            {
                return Err(VizuaraError::RenderError(format!(
                    "Text render failed: {}",
                    e
                )));
            }
        }
        Ok(())
    }

    /// 将图元转换为顶点数据，同时收集文本
    fn primitives_to_vertices_collect_text(
        &self,
        primitives: &[Primitive],
        styles: &[Style],
        texts: &mut Vec<(String, f32, f32, f32, Color, HorizontalAlign, VerticalAlign)>,
    ) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for (i, primitive) in primitives.iter().enumerate() {
            // 当样式数量少于图元数量时，使用默认样式兜底，避免丢弃后续图元
            let style = styles.get(i).cloned().unwrap_or_else(Style::default);
            match primitive {
                Primitive::Point(point) => {
                    // 将点渲染为小三角形
                    let size = style.marker_size / 100.0; // 标准化大小
                    let color = style.fill_color.unwrap_or(Color::BLUE);
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];

                    // 将数据坐标转换为 NDC 坐标 (-1 到 1)
                    let x = (point.x / self.size.width as f32) * 2.0 - 1.0;
                    let y = 1.0 - (point.y / self.size.height as f32) * 2.0;

                    vertices.extend_from_slice(&[
                        Vertex::new([x, y + size], color_array),
                        Vertex::new([x - size, y - size], color_array),
                        Vertex::new([x + size, y - size], color_array),
                    ]);
                }
                Primitive::Points(points) => {
                    let size = style.marker_size / 100.0;
                    let color = style.fill_color.unwrap_or(Color::BLUE);
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];

                    for point in points {
                        let x = (point.x / self.size.width as f32) * 2.0 - 1.0;
                        let y = 1.0 - (point.y / self.size.height as f32) * 2.0;

                        vertices.extend_from_slice(&[
                            Vertex::new([x, y + size], color_array),
                            Vertex::new([x - size, y - size], color_array),
                            Vertex::new([x + size, y - size], color_array),
                        ]);
                    }
                }
                Primitive::Line { start, end } => {
                    // 使用描边颜色，回退到填充色
                    let color = style
                        .stroke_color
                        .or(style.fill_color)
                        .unwrap_or(Color::WHITE);
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];

                    // 线宽（像素）转换为偏移（像素）
                    let half_w = (style.stroke_width.max(1.0)) / 2.0;

                    // 计算法线偏移（像素空间）
                    let dx = end.x - start.x;
                    let dy = end.y - start.y;
                    let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                    let nx = -dy / len;
                    let ny = dx / len;
                    let ox = nx * half_w;
                    let oy = ny * half_w;

                    // 四个角（像素空间）
                    let p0 = (start.x + ox, start.y + oy);
                    let p1 = (end.x + ox, end.y + oy);
                    let p2 = (end.x - ox, end.y - oy);
                    let p3 = (start.x - ox, start.y - oy);

                    // 像素转 NDC
                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    let v0 = to_ndc(p0);
                    let v1 = to_ndc(p1);
                    let v2 = to_ndc(p2);
                    let v3 = to_ndc(p3);

                    // 两个三角形
                    vertices.extend_from_slice(&[
                        Vertex::new(v0, color_array),
                        Vertex::new(v1, color_array),
                        Vertex::new(v2, color_array),
                        Vertex::new(v0, color_array),
                        Vertex::new(v2, color_array),
                        Vertex::new(v3, color_array),
                    ]);
                }
                Primitive::LineStrip(points) => {
                    if points.len() < 2 {
                        continue;
                    }
                    let color = style
                        .stroke_color
                        .or(style.fill_color)
                        .unwrap_or(Color::WHITE);
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];
                    let half_w = (style.stroke_width.max(1.0)) / 2.0;

                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    for seg in points.windows(2) {
                        let a = seg[0];
                        let b = seg[1];
                        let dx = b.x - a.x;
                        let dy = b.y - a.y;
                        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                        let nx = -dy / len;
                        let ny = dx / len;
                        let ox = nx * half_w;
                        let oy = ny * half_w;

                        let p0 = (a.x + ox, a.y + oy);
                        let p1 = (b.x + ox, b.y + oy);
                        let p2 = (b.x - ox, b.y - oy);
                        let p3 = (a.x - ox, a.y - oy);

                        let v0 = to_ndc(p0);
                        let v1 = to_ndc(p1);
                        let v2 = to_ndc(p2);
                        let v3 = to_ndc(p3);

                        vertices.extend_from_slice(&[
                            Vertex::new(v0, color_array),
                            Vertex::new(v1, color_array),
                            Vertex::new(v2, color_array),
                            Vertex::new(v0, color_array),
                            Vertex::new(v2, color_array),
                            Vertex::new(v3, color_array),
                        ]);
                    }
                }
                Primitive::Rectangle { min, max } => {
                    // 使用填充颜色渲染矩形（两个三角形）
                    let color = style.fill_color.unwrap_or(Color::WHITE);
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];

                    // 四个角（像素坐标）映射为 NDC
                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    let x0 = min.x.min(max.x);
                    let y0 = min.y.min(max.y);
                    let x1 = max.x.max(min.x);
                    let y1 = max.y.max(min.y);

                    let tl = to_ndc((x0, y0)); // top-left
                    let tr = to_ndc((x1, y0)); // top-right
                    let bl = to_ndc((x0, y1)); // bottom-left
                    let br = to_ndc((x1, y1)); // bottom-right

                    // 两个三角形填充矩形（在关闭 cull 的情况下，无需严格关心缠绕方向）
                    vertices.extend_from_slice(&[
                        // 三角形 1: tl, bl, br
                        Vertex::new(tl, color_array),
                        Vertex::new(bl, color_array),
                        Vertex::new(br, color_array),
                        // 三角形 2: tl, br, tr
                        Vertex::new(tl, color_array),
                        Vertex::new(br, color_array),
                        Vertex::new(tr, color_array),
                    ]);

                    // 如果需要描边，可在此追加四条边为细线，但当前仅填充
                }
                Primitive::RectangleStyled {
                    min,
                    max,
                    fill,
                    stroke,
                } => {
                    // 填充
                    let color_array = [fill.r, fill.g, fill.b, fill.a * style.opacity];

                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    let x0 = min.x.min(max.x);
                    let y0 = min.y.min(max.y);
                    let x1 = max.x.max(min.x);
                    let y1 = max.y.max(min.y);

                    let tl = to_ndc((x0, y0));
                    let tr = to_ndc((x1, y0));
                    let bl = to_ndc((x0, y1));
                    let br = to_ndc((x1, y1));

                    vertices.extend_from_slice(&[
                        Vertex::new(tl, color_array),
                        Vertex::new(bl, color_array),
                        Vertex::new(br, color_array),
                        Vertex::new(tl, color_array),
                        Vertex::new(br, color_array),
                        Vertex::new(tr, color_array),
                    ]);

                    // 描边（如果有）
                    if let Some((stroke_color, stroke_w)) = stroke {
                        let style_line = Style::new().stroke(*stroke_color, *stroke_w);
                        let mut dummy_texts: Vec<(
                            String,
                            f32,
                            f32,
                            f32,
                            Color,
                            HorizontalAlign,
                            VerticalAlign,
                        )> = Vec::new();
                        // 左
                        vertices.extend(self.primitives_to_vertices_collect_text(
                            &[Primitive::Line {
                                start: nalgebra::Point2::new(x0, y0),
                                end: nalgebra::Point2::new(x0, y1),
                            }],
                            std::slice::from_ref(&style_line),
                            &mut dummy_texts,
                        ));
                        // 右
                        vertices.extend(self.primitives_to_vertices_collect_text(
                            &[Primitive::Line {
                                start: nalgebra::Point2::new(x1, y0),
                                end: nalgebra::Point2::new(x1, y1),
                            }],
                            std::slice::from_ref(&style_line),
                            &mut dummy_texts,
                        ));
                        // 上
                        vertices.extend(self.primitives_to_vertices_collect_text(
                            &[Primitive::Line {
                                start: nalgebra::Point2::new(x0, y0),
                                end: nalgebra::Point2::new(x1, y0),
                            }],
                            std::slice::from_ref(&style_line),
                            &mut dummy_texts,
                        ));
                        // 下
                        vertices.extend(self.primitives_to_vertices_collect_text(
                            &[Primitive::Line {
                                start: nalgebra::Point2::new(x0, y1),
                                end: nalgebra::Point2::new(x1, y1),
                            }],
                            std::slice::from_ref(&style_line),
                            &mut dummy_texts,
                        ));
                    }
                }
                Primitive::Polyline {
                    points,
                    color,
                    width,
                } => {
                    if points.len() < 2 {
                        continue;
                    }
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];
                    let half_w = (width.max(1.0)) / 2.0;

                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    for seg in points.windows(2) {
                        let start = &seg[0];
                        let end = &seg[1];

                        let dx = end.x - start.x;
                        let dy = end.y - start.y;
                        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                        let nx = -dy / len;
                        let ny = dx / len;
                        let ox = nx * half_w;
                        let oy = ny * half_w;

                        let p0 = (start.x + ox, start.y + oy);
                        let p1 = (end.x + ox, end.y + oy);
                        let p2 = (end.x - ox, end.y - oy);
                        let p3 = (start.x - ox, start.y - oy);

                        let v0 = to_ndc(p0);
                        let v1 = to_ndc(p1);
                        let v2 = to_ndc(p2);
                        let v3 = to_ndc(p3);

                        vertices.extend_from_slice(&[
                            Vertex::new(v0, color_array),
                            Vertex::new(v1, color_array),
                            Vertex::new(v2, color_array),
                            Vertex::new(v0, color_array),
                            Vertex::new(v2, color_array),
                            Vertex::new(v3, color_array),
                        ]);
                    }
                }
                Primitive::Polygon {
                    points,
                    fill,
                    stroke,
                } => {
                    if points.len() < 3 {
                        continue;
                    }

                    let fill_color_array = [fill.r, fill.g, fill.b, fill.a * style.opacity];

                    let to_ndc = |point: &nalgebra::Point2<f32>| -> [f32; 2] {
                        let x = (point.x / self.size.width as f32) * 2.0 - 1.0;
                        let y = 1.0 - (point.y / self.size.height as f32) * 2.0;
                        [x, y]
                    };

                    // 简单的扇形三角化：从第一个点向所有其他点连线
                    let first_vertex = to_ndc(&points[0]);
                    for i in 1..points.len() - 1 {
                        let v1 = to_ndc(&points[i]);
                        let v2 = to_ndc(&points[i + 1]);

                        vertices.extend_from_slice(&[
                            Vertex::new(first_vertex, fill_color_array),
                            Vertex::new(v1, fill_color_array),
                            Vertex::new(v2, fill_color_array),
                        ]);
                    }

                    // 如果有边框，绘制边框
                    if let Some((stroke_color, stroke_width)) = stroke {
                        let stroke_color_array = [
                            stroke_color.r,
                            stroke_color.g,
                            stroke_color.b,
                            stroke_color.a * style.opacity,
                        ];
                        let half_w = (stroke_width.max(1.0)) / 2.0;

                        for i in 0..points.len() {
                            let start = &points[i];
                            let end = &points[(i + 1) % points.len()]; // 闭合多边形

                            let dx = end.x - start.x;
                            let dy = end.y - start.y;
                            let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                            let nx = -dy / len;
                            let ny = dx / len;
                            let ox = nx * half_w;
                            let oy = ny * half_w;

                            let p0 = (start.x + ox, start.y + oy);
                            let p1 = (end.x + ox, end.y + oy);
                            let p2 = (end.x - ox, end.y - oy);
                            let p3 = (start.x - ox, start.y - oy);

                            let to_ndc_pt = |(x, y): (f32, f32)| -> [f32; 2] {
                                let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                                let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                                [xn, yn]
                            };

                            let v0 = to_ndc_pt(p0);
                            let v1 = to_ndc_pt(p1);
                            let v2 = to_ndc_pt(p2);
                            let v3 = to_ndc_pt(p3);

                            vertices.extend_from_slice(&[
                                Vertex::new(v0, stroke_color_array),
                                Vertex::new(v1, stroke_color_array),
                                Vertex::new(v2, stroke_color_array),
                                Vertex::new(v0, stroke_color_array),
                                Vertex::new(v2, stroke_color_array),
                                Vertex::new(v3, stroke_color_array),
                            ]);
                        }
                    }
                }
                Primitive::ArcSector {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                    fill,
                    stroke,
                } => {
                    let segments = ((end_angle - start_angle).abs() * 32.0 / std::f32::consts::PI)
                        .max(8.0) as usize;
                    let fill_color_array = [fill.r, fill.g, fill.b, fill.a * style.opacity];

                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    let center_ndc = to_ndc((center.x, center.y));

                    // 生成扇形三角形
                    for i in 0..segments {
                        let angle1 = start_angle
                            + (i as f32) * (end_angle - start_angle) / (segments as f32);
                        let angle2 = start_angle
                            + ((i + 1) as f32) * (end_angle - start_angle) / (segments as f32);

                        let x1 = center.x + radius * angle1.cos();
                        let y1 = center.y + radius * angle1.sin();
                        let x2 = center.x + radius * angle2.cos();
                        let y2 = center.y + radius * angle2.sin();

                        let v1 = to_ndc((x1, y1));
                        let v2 = to_ndc((x2, y2));

                        vertices.extend_from_slice(&[
                            Vertex::new(center_ndc, fill_color_array),
                            Vertex::new(v1, fill_color_array),
                            Vertex::new(v2, fill_color_array),
                        ]);
                    }

                    // 如果有边框，绘制边框
                    if let Some((stroke_color, stroke_width)) = stroke {
                        let stroke_color_array = [
                            stroke_color.r,
                            stroke_color.g,
                            stroke_color.b,
                            stroke_color.a * style.opacity,
                        ];
                        let half_w = (stroke_width.max(1.0)) / 2.0;

                        // 绘制弧线边框
                        for i in 0..segments {
                            let angle1 = start_angle
                                + (i as f32) * (end_angle - start_angle) / (segments as f32);
                            let angle2 = start_angle
                                + ((i + 1) as f32) * (end_angle - start_angle) / (segments as f32);

                            let x1 = center.x + radius * angle1.cos();
                            let y1 = center.y + radius * angle1.sin();
                            let x2 = center.x + radius * angle2.cos();
                            let y2 = center.y + radius * angle2.sin();

                            // 简化：用线段近似边框
                            let dx = x2 - x1;
                            let dy = y2 - y1;
                            let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                            let nx = -dy / len;
                            let ny = dx / len;
                            let ox = nx * half_w;
                            let oy = ny * half_w;

                            let p0 = (x1 + ox, y1 + oy);
                            let p1 = (x2 + ox, y2 + oy);
                            let p2 = (x2 - ox, y2 - oy);
                            let p3 = (x1 - ox, y1 - oy);

                            let v0 = to_ndc(p0);
                            let v1 = to_ndc(p1);
                            let v2 = to_ndc(p2);
                            let v3 = to_ndc(p3);

                            vertices.extend_from_slice(&[
                                Vertex::new(v0, stroke_color_array),
                                Vertex::new(v1, stroke_color_array),
                                Vertex::new(v2, stroke_color_array),
                                Vertex::new(v0, stroke_color_array),
                                Vertex::new(v2, stroke_color_array),
                                Vertex::new(v3, stroke_color_array),
                            ]);
                        }
                    }
                }
                Primitive::ArcRing {
                    center,
                    inner_radius,
                    outer_radius,
                    start_angle,
                    end_angle,
                    fill,
                    stroke,
                } => {
                    let segments = ((end_angle - start_angle).abs() * 32.0 / std::f32::consts::PI)
                        .max(8.0) as usize;
                    let fill_color_array = [fill.r, fill.g, fill.b, fill.a * style.opacity];

                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    // 生成圆环四边形
                    for i in 0..segments {
                        let angle1 = start_angle
                            + (i as f32) * (end_angle - start_angle) / (segments as f32);
                        let angle2 = start_angle
                            + ((i + 1) as f32) * (end_angle - start_angle) / (segments as f32);

                        // 内环点
                        let inner_x1 = center.x + inner_radius * angle1.cos();
                        let inner_y1 = center.y + inner_radius * angle1.sin();
                        let inner_x2 = center.x + inner_radius * angle2.cos();
                        let inner_y2 = center.y + inner_radius * angle2.sin();

                        // 外环点
                        let outer_x1 = center.x + outer_radius * angle1.cos();
                        let outer_y1 = center.y + outer_radius * angle1.sin();
                        let outer_x2 = center.x + outer_radius * angle2.cos();
                        let outer_y2 = center.y + outer_radius * angle2.sin();

                        let inner_v1 = to_ndc((inner_x1, inner_y1));
                        let inner_v2 = to_ndc((inner_x2, inner_y2));
                        let outer_v1 = to_ndc((outer_x1, outer_y1));
                        let outer_v2 = to_ndc((outer_x2, outer_y2));

                        // 两个三角形组成四边形
                        vertices.extend_from_slice(&[
                            Vertex::new(inner_v1, fill_color_array),
                            Vertex::new(outer_v1, fill_color_array),
                            Vertex::new(outer_v2, fill_color_array),
                            Vertex::new(inner_v1, fill_color_array),
                            Vertex::new(outer_v2, fill_color_array),
                            Vertex::new(inner_v2, fill_color_array),
                        ]);
                    }

                    // 边框渲染（简化处理）
                    if let Some((stroke_color, stroke_width)) = stroke {
                        let stroke_color_array = [
                            stroke_color.r,
                            stroke_color.g,
                            stroke_color.b,
                            stroke_color.a * style.opacity,
                        ];
                        let half_w = (stroke_width.max(1.0)) / 2.0;

                        // 简化：只绘制内外圆弧边框
                        for i in 0..segments {
                            let angle1 = start_angle
                                + (i as f32) * (end_angle - start_angle) / (segments as f32);
                            let angle2 = start_angle
                                + ((i + 1) as f32) * (end_angle - start_angle) / (segments as f32);

                            // 外环线段
                            let outer_x1 = center.x + outer_radius * angle1.cos();
                            let outer_y1 = center.y + outer_radius * angle1.sin();
                            let outer_x2 = center.x + outer_radius * angle2.cos();
                            let outer_y2 = center.y + outer_radius * angle2.sin();

                            let dx = outer_x2 - outer_x1;
                            let dy = outer_y2 - outer_y1;
                            let len = (dx * dx + dy * dy).sqrt().max(1e-6);
                            let nx = -dy / len;
                            let ny = dx / len;
                            let ox = nx * half_w;
                            let oy = ny * half_w;

                            let p0 = (outer_x1 + ox, outer_y1 + oy);
                            let p1 = (outer_x2 + ox, outer_y2 + oy);
                            let p2 = (outer_x2 - ox, outer_y2 - oy);
                            let p3 = (outer_x1 - ox, outer_y1 - oy);

                            let v0 = to_ndc(p0);
                            let v1 = to_ndc(p1);
                            let v2 = to_ndc(p2);
                            let v3 = to_ndc(p3);

                            vertices.extend_from_slice(&[
                                Vertex::new(v0, stroke_color_array),
                                Vertex::new(v1, stroke_color_array),
                                Vertex::new(v2, stroke_color_array),
                                Vertex::new(v0, stroke_color_array),
                                Vertex::new(v2, stroke_color_array),
                                Vertex::new(v3, stroke_color_array),
                            ]);
                        }
                    }
                }
                Primitive::Circle { center, radius } => {
                    let segments = 32; // 圆的分段数
                    let color = style.fill_color.unwrap_or(Color::BLUE);
                    let color_array = [color.r, color.g, color.b, color.a * style.opacity];

                    let to_ndc = |(x, y): (f32, f32)| -> [f32; 2] {
                        let xn = (x / self.size.width as f32) * 2.0 - 1.0;
                        let yn = 1.0 - (y / self.size.height as f32) * 2.0;
                        [xn, yn]
                    };

                    let center_ndc = to_ndc((center.x, center.y));

                    // 生成圆形三角形
                    for i in 0..segments {
                        let angle1 = (i as f32) * 2.0 * std::f32::consts::PI / (segments as f32);
                        let angle2 =
                            ((i + 1) as f32) * 2.0 * std::f32::consts::PI / (segments as f32);

                        let x1 = center.x + radius * angle1.cos();
                        let y1 = center.y + radius * angle1.sin();
                        let x2 = center.x + radius * angle2.cos();
                        let y2 = center.y + radius * angle2.sin();

                        let v1 = to_ndc((x1, y1));
                        let v2 = to_ndc((x2, y2));

                        vertices.extend_from_slice(&[
                            Vertex::new(center_ndc, color_array),
                            Vertex::new(v1, color_array),
                            Vertex::new(v2, color_array),
                        ]);
                    }
                }
                Primitive::Text {
                    position,
                    content,
                    size,
                    color,
                    h_align,
                    v_align,
                } => {
                    // 收集文本，实际绘制在 glyphon pass 中（克隆内容以延长生命周期）
                    texts.push((
                        content.clone(),
                        position.x,
                        position.y,
                        *size,
                        *color,
                        *h_align,
                        *v_align,
                    ));
                }
                // 其他图元类型暂不渲染（如 Circle 等）
                _ => {}
            }
        }

        vertices
    }
}
