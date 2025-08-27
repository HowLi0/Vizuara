use wgpu::util::DeviceExt;
use winit::window::Window;
use vizuara_core::{Result, VizuaraError};
use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Point3, Vector3};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex3D {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex3D {
    pub fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Uniforms {
    view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, view: &Matrix4<f32>, proj: &Matrix4<f32>) {
        self.view_proj = (proj * view).into();
    }
}

/// 3D WGPU 渲染器
pub struct Wgpu3DRenderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    
    // 深度缓冲
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    
    // 统一缓冲区
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,
    
    // 相机参数
    camera_eye: Point3<f32>,
    camera_target: Point3<f32>,
    camera_up: Vector3<f32>,
    fov: f32,
    near: f32,
    far: f32,
}

impl Wgpu3DRenderer {
    /// 创建新的3D渲染器
    pub async fn new(
        window: &Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Result<(Self, wgpu::Surface<'_>)> {
        // 创建wgpu实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)
            .map_err(|e| VizuaraError::RenderError(format!("Failed to create surface: {}", e)))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| VizuaraError::RenderError("Failed to find adapter".to_string()))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(|e| VizuaraError::RenderError(format!("Failed to create device: {}", e)))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // 创建深度纹理
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 创建统一缓冲区
        let uniforms = Uniforms::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Uniforms>() as u64),
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        // 创建着色器
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3D Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader_3d.wgsl").into()),
        });

        // 创建渲染管线
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("3D Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3D Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex3D>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // 初始相机参数
        let camera_eye = Point3::new(4.0, 3.0, 2.0);
        let camera_target = Point3::new(0.0, 0.0, 0.0);
        let camera_up = Vector3::new(0.0, 1.0, 0.0);
        let fov = 45.0_f32.to_radians();
        let near = 0.1;
        let far = 100.0;

        let mut renderer = Self {
            device,
            queue,
            config,
            size,
            render_pipeline,
            depth_texture,
            depth_view,
            uniform_buffer,
            uniform_bind_group,
            uniforms,
            camera_eye,
            camera_target,
            camera_up,
            fov,
            near,
            far,
        };

        renderer.update_uniforms();

        Ok((renderer, surface))
    }

    /// 更新统一缓冲区
    fn update_uniforms(&mut self) {
        let aspect = self.size.width as f32 / self.size.height as f32;
        let proj = Matrix4::new_perspective(aspect, self.fov, self.near, self.far);
        let view = Matrix4::look_at_rh(&self.camera_eye, &self.camera_target, &self.camera_up);
        
        self.uniforms.update_view_proj(&view, &proj);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    /// 设置相机位置
    pub fn set_camera(&mut self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) {
        self.camera_eye = eye;
        self.camera_target = target;
        self.camera_up = up;
        self.update_uniforms();
    }

    /// 旋转相机
    pub fn rotate_camera(&mut self, delta_x: f32, delta_y: f32) {
        let distance = (self.camera_eye - self.camera_target).magnitude();
        
        // 球坐标系旋转
        let mut theta = (self.camera_eye.x - self.camera_target.x).atan2(self.camera_eye.z - self.camera_target.z);
        let mut phi = ((self.camera_eye.y - self.camera_target.y) / distance).acos();
        
        theta -= delta_x * 0.01;
        phi = (phi + delta_y * 0.01).clamp(0.1, std::f32::consts::PI - 0.1);
        
        self.camera_eye.x = self.camera_target.x + distance * phi.sin() * theta.cos();
        self.camera_eye.z = self.camera_target.z + distance * phi.sin() * theta.sin();
        self.camera_eye.y = self.camera_target.y + distance * phi.cos();
        
        self.update_uniforms();
    }

    /// 缩放相机
    pub fn zoom_camera(&mut self, delta: f32) {
        let direction = (self.camera_eye - self.camera_target).normalize();
        let distance = (self.camera_eye - self.camera_target).magnitude();
        let new_distance = (distance + delta * 0.5).clamp(1.0, 50.0);
        
        self.camera_eye = self.camera_target + direction * new_distance;
        self.update_uniforms();
    }

    /// 调整窗口大小
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            
            // 重新创建深度纹理
            self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("depth_texture"),
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            
            self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.update_uniforms();
        }
    }

    /// 渲染3D场景
    pub fn render_3d(&mut self, surface: &wgpu::Surface, vertices: &[Vertex3D], indices: &[u16]) -> Result<()> {
        let output = surface
            .get_current_texture()
            .map_err(|e| VizuaraError::RenderError(format!("Failed to get surface texture: {}", e)))?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 创建顶点缓冲区
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // 创建索引缓冲区
        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
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
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
