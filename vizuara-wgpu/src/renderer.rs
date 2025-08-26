use wgpu::util::DeviceExt;
use winit::window::Window;
use vizuara_core::{Color, Primitive, Style, Result, VizuaraError};
use bytemuck::{Pod, Zeroable};
//use nalgebra::Point2;

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
}

impl WgpuRenderer {
    /// 创建新的渲染器
    pub async fn new(
        window: &Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Result<(Self, wgpu::Surface<'_>)> {
        // 尝试不同后端以适配更多环境（优先 GL，再尝试 Vulkan）
        let backend_candidates = [wgpu::Backends::GL, wgpu::Backends::VULKAN, wgpu::Backends::PRIMARY];

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
                .get(0)
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

            let renderer = WgpuRenderer {
                _instance: instance,
                _adapter: adapter,
                device,
                queue,
                config,
                size,
                render_pipeline,
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
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/basic.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
                cull_mode: Some(wgpu::Face::Back),
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

    /// 调整窗口大小
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface: &wgpu::Surface) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            surface.configure(&self.device, &self.config);
        }
    }

    /// 渲染帧
    pub fn render(&mut self, surface: &wgpu::Surface, primitives: &[Primitive], styles: &[Style]) -> Result<()> {
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
                return Err(VizuaraError::RenderError(format!("Failed to get surface texture: {}", e)));
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // 转换图元为顶点
        let vertices = self.primitives_to_vertices(primitives, styles);
        
        if !vertices.is_empty() {
            let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
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
        } else {
            // 即使没有顶点也要清屏
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// 将图元转换为顶点数据
    fn primitives_to_vertices(&self, primitives: &[Primitive], styles: &[Style]) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        
        for (primitive, style) in primitives.iter().zip(styles.iter()) {
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
                // 其他图元类型的实现...
                _ => {
                    // 暂时跳过未实现的图元类型
                }
            }
        }
        
        vertices
    }
}

