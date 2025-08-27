//! 支持光照的高级3D渲染器
//! 
//! 基于物理的渲染(PBR)和多光源系统

use nalgebra::{Matrix4, Vector3, Point3};
use wgpu::{
    Buffer, BindGroup, BindGroupLayout, RenderPipeline, Surface, SurfaceConfiguration,
    util::DeviceExt, BufferUsages, ShaderStages, BindingType, BufferBindingType,
};
use winit::window::Window;
use vizuara_core::{Result, VizuaraError};
use vizuara_3d::{Light, LightType, Material};

/// 支持法向量的顶点结构
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3DLit {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex3DLit {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // normal
        2 => Float32x3, // color
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex3DLit>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// 相机统一缓冲区
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    camera_position: [f32; 3],
    _padding: f32,
}

/// GPU光源数据 (WGSL 16字节对齐，调整到75字节)
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    position: [f32; 3],      // 12 bytes
    light_type: f32,         // 4 bytes
    direction: [f32; 3],     // 12 bytes  
    intensity: f32,          // 4 bytes
    color: [f32; 3],         // 12 bytes
    enabled: f32,            // 4 bytes
    radius: f32,             // 4 bytes
    inner_angle: f32,        // 4 bytes
    _padding: [f32; 2],      // 8 bytes
    _extra_pad: [f32; 3],    // 12 bytes
    _pad_end: f32,           // 4 bytes，显式补齐到 80 字节
}

/// 光照统一缓冲区 (WGSL 16字节对齐，24字节头部)
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LightingUniform {
    ambient_color: [f32; 3],   // 12 bytes
    ambient_intensity: f32,    // 4 bytes -> 16 bytes
    num_lights: f32,           // 4 bytes
    _padding: f32,             // 4 bytes
    _padding2: [f32; 2],       // 8 bytes -> 32 字节头部（去除隐式填充）
    lights: [LightUniform; 8], // 8 * 80字节 = 640字节，总共672字节
}

/// 材质统一缓冲区 (WGSL 16字节对齐)
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialUniform {
    albedo: [f32; 3],
    metallic: f32,
    roughness: f32,
    ao: f32,
    _padding1: [f32; 2],
    emissive: [f32; 3],
    _padding2: f32,
}

// 注意：通过显式 padding 确保 Rust 端与 WGSL 的布局一致：
// - LightUniform: 80 bytes
// - LightingUniform: 32 (header) + 8*80 = 672 bytes

/// 支持光照的3D渲染器
pub struct Wgpu3DLitRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter, // 保存adapter引用
    
    // 管线
    render_pipeline: RenderPipeline,
    
    // 绑定组布局
    _camera_bind_group_layout: BindGroupLayout,
    _lighting_bind_group_layout: BindGroupLayout,
    _material_bind_group_layout: BindGroupLayout,
    
    // 统一缓冲区
    camera_buffer: Buffer,
    lighting_buffer: Buffer,
    material_buffer: Buffer,
    
    // 绑定组
    camera_bind_group: BindGroup,
    lighting_bind_group: BindGroup,
    material_bind_group: BindGroup,
    
    // 相机参数
    camera_position: Point3<f32>,
    camera_rotation: (f32, f32), // (yaw, pitch)
    camera_distance: f32,
    
    // 光照系统
    lights: Vec<Light>,
    ambient_color: [f32; 3],
    ambient_intensity: f32,
}

impl Wgpu3DLitRenderer {
    /// 创建新的光照渲染器
    pub async fn new(window: &Window, size: winit::dpi::PhysicalSize<u32>) -> Result<(Self, Surface<'_>)> {
        // 创建wgpu实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        // 创建表面
        let surface = instance.create_surface(window)
            .map_err(|e| VizuaraError::RenderError(format!("Failed to create surface: {}", e)))?;

        // 请求适配器
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| VizuaraError::RenderError("Failed to find adapter".to_string()))?;

        // 请求设备和队列
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| VizuaraError::RenderError(format!("Failed to request device: {}", e)))?;

        // 配置表面
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
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

        // 创建着色器
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3D Lit Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader_3d_lit.wgsl").into()),
        });

        // 创建绑定组布局
    let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<CameraUniform>() as u64),
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

    let lighting_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<LightingUniform>() as u64),
                },
                count: None,
            }],
            label: Some("lighting_bind_group_layout"),
        });

    let material_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<MaterialUniform>() as u64),
                },
                count: None,
            }],
            label: Some("material_bind_group_layout"),
        });

        // 创建统一缓冲区
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let lighting_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Lighting Buffer"),
            size: std::mem::size_of::<LightingUniform>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let material_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Material Buffer"),
            size: std::mem::size_of::<MaterialUniform>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // 创建绑定组
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &lighting_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: lighting_buffer.as_entire_binding(),
            }],
            label: Some("lighting_bind_group"),
        });

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: material_buffer.as_entire_binding(),
            }],
            label: Some("material_bind_group"),
        });

        // 创建渲染管线
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &lighting_bind_group_layout,
                &material_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3D Lit Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex3DLit::desc()],
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
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
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

        // 初始化默认值
        let camera_position = Point3::new(0.0, 0.0, 5.0);
        let camera_rotation = (0.0, 0.0);
        let camera_distance = 5.0;
        
        let lights = Light::default_scene();
        let ambient_color = [0.1, 0.1, 0.15];
        let ambient_intensity = 0.3;

        let renderer = Self {
            device,
            queue,
            adapter,
            render_pipeline,
            _camera_bind_group_layout: camera_bind_group_layout,
            _lighting_bind_group_layout: lighting_bind_group_layout,
            _material_bind_group_layout: material_bind_group_layout,
            camera_buffer,
            lighting_buffer,
            material_buffer,
            camera_bind_group,
            lighting_bind_group,
            material_bind_group,
            camera_position,
            camera_rotation,
            camera_distance,
            lights,
            ambient_color,
            ambient_intensity,
        };

        // 初始化统一缓冲区
        renderer.update_camera_buffer(size.width as f32 / size.height as f32);
        renderer.update_lighting_buffer();
        renderer.update_material_buffer(&Material::data_visualization()[0]); // 使用默认材质

        Ok((renderer, surface))
    }

    /// 更新相机缓冲区
    fn update_camera_buffer(&self, aspect_ratio: f32) {
        // 计算视图矩阵
        let view = Matrix4::look_at_rh(
            &self.camera_position,
            &Point3::origin(),
            &Vector3::y(),
        );

        // 计算投影矩阵
        let proj = Matrix4::new_perspective(aspect_ratio, 45.0_f32.to_radians(), 0.1, 100.0);

        let camera_uniform = CameraUniform {
            view_proj: (proj * view).into(),
            camera_position: self.camera_position.coords.into(),
            _padding: 0.0,
        };

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    /// 更新光照缓冲区
    fn update_lighting_buffer(&self) {
        let mut light_uniforms = [LightUniform {
            position: [0.0; 3],
            light_type: 0.0,
            direction: [0.0; 3],
            intensity: 0.0,
            color: [0.0; 3],
            enabled: 0.0,
            radius: 0.0,
            inner_angle: 0.0,
            _padding: [0.0; 2],
            _extra_pad: [0.0; 3],
            _pad_end: 0.0,
        }; 8];

        for (i, light) in self.lights.iter().take(8).enumerate() {
            let (position, direction, light_type_id, radius, inner_angle) = match &light.light_type {
                LightType::Directional { direction } => {
                    ([0.0; 3], [direction.x, direction.y, direction.z], 0.0, 0.0, 0.0)
                }
                LightType::Point { position, radius } => {
                    (position.coords.into(), [0.0; 3], 1.0, *radius, 0.0)
                }
                LightType::Spot { position, direction, inner_angle, outer_angle } => {
                    (position.coords.into(), [direction.x, direction.y, direction.z], 2.0, *outer_angle, *inner_angle)
                }
            };

            light_uniforms[i] = LightUniform {
                position,
                light_type: light_type_id,
                direction,
                intensity: light.intensity,
                color: [light.color.r, light.color.g, light.color.b],
                enabled: if light.enabled { 1.0 } else { 0.0 },
                radius,
                inner_angle,
                _padding: [0.0; 2],
                _extra_pad: [0.0; 3],
                _pad_end: 0.0,
            };
        }

        let lighting_uniform = LightingUniform {
            ambient_color: self.ambient_color,
            ambient_intensity: self.ambient_intensity,
            num_lights: self.lights.len().min(8) as f32,
            _padding: 0.0,
            _padding2: [0.0; 2],
            lights: light_uniforms,
        };

        let binding = [lighting_uniform];
        let buffer_data = bytemuck::cast_slice(&binding);
        println!("🔧 Lighting buffer size: {} bytes", buffer_data.len());
        println!("🔧 LightUniform size: {} bytes", std::mem::size_of::<LightUniform>());
        println!("🔧 LightingUniform size: {} bytes", std::mem::size_of::<LightingUniform>());
        
        self.queue.write_buffer(
            &self.lighting_buffer,
            0,
            buffer_data,
        );
    }

    /// 更新材质缓冲区
    fn update_material_buffer(&self, material: &Material) {
        let material_uniform = MaterialUniform {
            albedo: [material.albedo.r, material.albedo.g, material.albedo.b],
            metallic: material.metallic,
            roughness: material.roughness,
            ao: material.ao,
            _padding1: [0.0; 2],
            emissive: [material.emissive.r, material.emissive.g, material.emissive.b],
            _padding2: 0.0,
        };

        self.queue.write_buffer(
            &self.material_buffer,
            0,
            bytemuck::cast_slice(&[material_uniform]),
        );
    }

    /// 设置相机位置
    pub fn set_camera_position(&mut self, position: Point3<f32>) {
        self.camera_position = position;
    }

    /// 旋转相机
    pub fn rotate_camera(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.camera_rotation.0 += delta_yaw;
        self.camera_rotation.1 = (self.camera_rotation.1 + delta_pitch).clamp(-1.5, 1.5);
        
        // 更新相机位置 (轨道相机)
        let cos_pitch = self.camera_rotation.1.cos();
        let sin_pitch = self.camera_rotation.1.sin();
        let cos_yaw = self.camera_rotation.0.cos();
        let sin_yaw = self.camera_rotation.0.sin();

        self.camera_position = Point3::new(
            self.camera_distance * cos_pitch * sin_yaw,
            self.camera_distance * sin_pitch,
            self.camera_distance * cos_pitch * cos_yaw,
        );
    }

    /// 缩放相机 (调整距离)
    pub fn zoom_camera(&mut self, factor: f32) {
        self.camera_distance = (self.camera_distance * factor).clamp(1.0, 50.0);
        
        // 更新相机位置
        let cos_pitch = self.camera_rotation.1.cos();
        let sin_pitch = self.camera_rotation.1.sin();
        let cos_yaw = self.camera_rotation.0.cos();
        let sin_yaw = self.camera_rotation.0.sin();

        self.camera_position = Point3::new(
            self.camera_distance * cos_pitch * sin_yaw,
            self.camera_distance * sin_pitch,
            self.camera_distance * cos_pitch * cos_yaw,
        );
    }

    /// 重置相机
    pub fn reset_camera(&mut self) {
        self.camera_position = Point3::new(0.0, 0.0, 5.0);
        self.camera_rotation = (0.0, 0.0);
        self.camera_distance = 5.0;
    }

    /// 添加光源
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// 设置环境光
    pub fn set_ambient_light(&mut self, color: [f32; 3], intensity: f32) {
        self.ambient_color = color;
        self.ambient_intensity = intensity;
    }

    /// 渲染帧
    pub fn render(
        &mut self,
        surface: &Surface,
        vertices: &[Vertex3DLit],
        indices: &[u16],
        material: &Material,
        aspect_ratio: f32,
    ) -> Result<()> {
        // 更新统一缓冲区
        self.update_camera_buffer(aspect_ratio);
        self.update_lighting_buffer();
        self.update_material_buffer(material);

        // 创建顶点和索引缓冲区
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        // 创建深度纹理
        let output = surface
            .get_current_texture()
            .map_err(|e| VizuaraError::RenderError(format!("Failed to get surface texture: {}", e)))?;
        
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: output.texture.width(),
                height: output.texture.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("depth_texture"),
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 获取当前帧
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 创建命令编码器
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // 开始渲染通道
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
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
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
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.lighting_bind_group, &[]);
            render_pass.set_bind_group(2, &self.material_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }

        // 提交命令
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// 调整渲染器大小
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface: &Surface) {
        if new_size.width > 0 && new_size.height > 0 {
            let surface_caps = surface.get_capabilities(&self.adapter);
            let config = SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_caps.formats[0],
                width: new_size.width,
                height: new_size.height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };
            surface.configure(&self.device, &config);
        }
    }
}
