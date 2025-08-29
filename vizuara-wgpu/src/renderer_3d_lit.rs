//! 支持光照的高级3D渲染器
//!
//! 基于物理的渲染(PBR)和多光源系统
use nalgebra::{Matrix4, Point3, Vector3, Vector4};
use vizuara_3d::{Axis3DDirection, Axis3DRenderData, CoordinateSystem3D, Light, LightType, Material};
use vizuara_core::{Color, Result, VizuaraError};
use glyphon::{
    Attrs, Buffer as GlyphBuffer, Family, FontSystem, Metrics, Resolution, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Wrap, Shaping,
};
use wgpu::{
    self, util::DeviceExt, BindGroup, BindGroupLayout, BindingType, Buffer, BufferBindingType,
    BufferUsages, RenderPipeline, ShaderStages, Surface, SurfaceConfiguration,
};
use winit::window::Window;
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

/// 3D文本顶点结构
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Text3DVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
    pub color: [f32; 4],
    pub char_code: u32,
}

impl Text3DVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x2, // tex_coord
        2 => Float32x4, // color
        3 => Uint32,    // char code
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Text3DVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn new(position: [f32; 3], tex_coord: [f32; 2], color: [f32; 4], char_code: u32) -> Self {
        Self { position, tex_coord, color, char_code }
    }
}

/// 坐标轴顶点结构（线条渲染）
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AxisVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl AxisVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // color
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<AxisVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

/// 坐标面顶点结构（带透明度）
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlaneVertex {
    pub position: [f32; 3],
    pub color: [f32; 4], // 包含透明度
}

impl PlaneVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x4, // color with alpha
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PlaneVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self { position, color }
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
    position: [f32; 3],   // 12 bytes
    light_type: f32,      // 4 bytes
    direction: [f32; 3],  // 12 bytes
    intensity: f32,       // 4 bytes
    color: [f32; 3],      // 12 bytes
    enabled: f32,         // 4 bytes
    radius: f32,          // 4 bytes
    inner_angle: f32,     // 4 bytes
    _padding: [f32; 2],   // 8 bytes
    _extra_pad: [f32; 3], // 12 bytes
    _pad_end: f32,        // 4 bytes，显式补齐到 80 字节
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
    axis_pipeline: RenderPipeline,
    plane_pipeline: RenderPipeline,
    text_pipeline: RenderPipeline,

    // Unicode 文本（屏幕空间覆盖）
    font_system: FontSystem,
    swash_cache: SwashCache,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer,
    // 简单的文本缓存，避免每帧重塑形
    text_cache: std::collections::HashMap<(String, u32), GlyphBuffer>,

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
    
    // 状态跟踪以避免不必要的更新
    camera_dirty: bool,
    lights_dirty: bool,
    last_aspect_ratio: f32,
}

impl Wgpu3DLitRenderer {
    /// 创建新的光照渲染器
    pub async fn new(
        window: &Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Result<(Self, Surface<'_>)> {
        // 创建wgpu实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        // 创建表面
        let surface = instance
            .create_surface(window)
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
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(
                            std::mem::size_of::<CameraUniform>() as u64,
                        ),
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let lighting_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                            LightingUniform,
                        >()
                            as u64),
                    },
                    count: None,
                }],
                label: Some("lighting_bind_group_layout"),
            });

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                            MaterialUniform,
                        >()
                            as u64),
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
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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

        // 创建坐标轴着色器
        let axis_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Axis Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/axis3d.wgsl").into()),
        });

        // 创建坐标轴管线布局（只需要相机绑定组）
        let axis_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Axis Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // 创建坐标轴渲染管线
        let axis_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Axis Render Pipeline"),
            layout: Some(&axis_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &axis_shader,
                entry_point: "vs_main",
                buffers: &[AxisVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &axis_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // 不剔除背面，因为线条没有面
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

        // 创建坐标面着色器
        let plane_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Plane Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/axis3d_plane.wgsl").into()),
        });

        // 创建坐标面渲染管线
        let plane_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Plane Render Pipeline"),
            layout: Some(&axis_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &plane_shader,
                entry_point: "vs_main",
                buffers: &[PlaneVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &plane_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // 不剔除，因为是半透明面
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false, // 半透明面不写入深度
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

        // 创建3D文本着色器
        let text_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3D Text Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/text3d.wgsl").into()),
        });

        // 创建3D文本渲染管线
        let text_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3D Text Render Pipeline"),
            layout: Some(&axis_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &text_shader,
                entry_point: "vs_main",
                buffers: &[Text3DVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &text_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // 不剔除，允许从各个角度查看
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

        // 初始化 glyphon 文本
        let mut font_system = FontSystem::new();
        {
            let db = font_system.db_mut();
            let font_candidates = [
                "/usr/share/fonts/truetype/noto/NotoSansSC-Regular.ttf",
                "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ];
            for path in font_candidates { let _ = db.load_font_file(path); }
        }
        let swash_cache = SwashCache::new();
        let mut text_atlas = TextAtlas::new(&device, &queue, config.format);
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
            None,
        );

        let renderer = Self {
            device,
            queue,
            adapter,
            render_pipeline,
            axis_pipeline,
            plane_pipeline,
            text_pipeline,
            font_system,
            swash_cache,
            text_atlas,
            text_renderer,
            text_cache: std::collections::HashMap::new(),
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
            camera_dirty: true,
            lights_dirty: true,
            last_aspect_ratio: size.width as f32 / size.height as f32,
        };

        // 初始化统一缓冲区
        renderer.update_camera_buffer(size.width as f32 / size.height as f32);
        renderer.update_lighting_buffer();
        renderer.update_material_buffer(&Material::data_visualization()[0]); // 使用默认材质

        Ok((renderer, surface))
    }

    // 将世界坐标投影为屏幕像素坐标
    fn world_to_screen(
        &self,
        p: Point3<f32>,
        aspect_ratio: f32,
        width: u32,
        height: u32,
    ) -> Option<(f32, f32)> {
        // 构造与 uniform 一致的视图投影
    let view = Matrix4::look_at_rh(&self.camera_position, &Point3::origin(), &Vector3::z());
        let proj = Matrix4::new_perspective(aspect_ratio, 45.0_f32.to_radians(), 0.1, 100.0);
        let mvp = proj * view;
        let hp = Vector4::new(p.x, p.y, p.z, 1.0);
        let cp = mvp * hp;
        if cp.w.abs() < 1e-6 { return None; }
        let ndc_x = cp.x / cp.w;
        let ndc_y = cp.y / cp.w;
        let ndc_z = cp.z / cp.w;
        if ndc_z < -1.0 || ndc_z > 1.0 { return None; }
        // 转屏幕像素
        let sx = (ndc_x * 0.5 + 0.5) * width as f32;
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * height as f32;
        Some((sx, sy))
    }

    fn draw_overlay_texts_for_axes(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        aspect: f32,
        width: u32,
        height: u32,
        render_data: &Axis3DRenderData,
    ) -> Result<()> {
        // 收集屏幕文本 (content, x, y, size, color)
        let mut texts: Vec<(String, f32, f32, f32, Color)> = Vec::new();

        // 刻度标签
        for (pos, content, _dir) in &render_data.tick_labels {
            if let Some((x, y)) = self.world_to_screen(*pos, aspect, width, height) {
                texts.push((content.clone(), x, y, 16.0, Color::WHITE));
            }
        }
        // 轴标题
        for (pos, content, _dir) in &render_data.axis_titles {
            if let Some((x, y)) = self.world_to_screen(*pos, aspect, width, height) {
                texts.push((content.clone(), x, y, 18.0, Color::rgb(1.0, 1.0, 0.8)));
            }
        }

        if texts.is_empty() { return Ok(()); }

        // 第一阶段：确保/更新缓存
        for (content, _x, _y, size, _color) in texts.iter() {
            let key = (content.clone(), *size as u32);
            if !self.text_cache.contains_key(&key) {
                let mut buf = GlyphBuffer::new(&mut self.font_system, Metrics::new(*size, *size));
                buf.set_size(&mut self.font_system, width as f32, height as f32);
                buf.set_text(
                    &mut self.font_system,
                    content,
                    Attrs::new().family(Family::SansSerif),
                    Shaping::Advanced,
                );
                buf.set_wrap(&mut self.font_system, Wrap::None);
                self.text_cache.insert(key, buf);
            }
        }

        // 构造 TextArea
        let mut areas: Vec<TextArea> = Vec::new();
        let to_u8 = |v: f32| -> u8 { (v.clamp(0.0, 1.0) * 255.0).round() as u8 };
        for (content, x, y, size, color) in &texts {
            let key = (content.clone(), *size as u32);
            let buf = self.text_cache.get(&key).expect("buffer exists");
            // 简易锚点：居中放置
            let width_est = content.chars().count() as f32 * if !content.is_ascii() { *size * 0.9 } else { *size * 0.6 };
            let em = *size;
            let left = *x - width_est / 2.0;
            let top = *y - em / 2.0;
            areas.push(TextArea{
                buffer: buf,
                left,
                top,
                scale: 1.0,
                bounds: TextBounds { left: 0, top: 0, right: width as i32, bottom: height as i32 },
                default_color: glyphon::Color::rgba(to_u8(color.r), to_u8(color.g), to_u8(color.b), to_u8(color.a)),
            });
        }

        // 准备 & 渲染
        if let Err(e) = self.text_renderer.prepare(
            &self.device, &self.queue, &mut self.font_system, &mut self.text_atlas,
            Resolution { width, height }, areas, &mut self.swash_cache,
        ) { return Err(VizuaraError::RenderError(format!("Text prepare failed: {}", e))); }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("3D Overlay Text Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations{ load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            if let Err(e) = self.text_renderer.render(&self.text_atlas, &mut render_pass) {
                return Err(VizuaraError::RenderError(format!("Text render failed: {}", e)));
            }
        }
        Ok(())
    }

    /// 更新相机缓冲区
    fn update_camera_buffer(&self, aspect_ratio: f32) {
        // 计算视图矩阵
    let view = Matrix4::look_at_rh(&self.camera_position, &Point3::origin(), &Vector3::z());

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
            let (position, direction, light_type_id, radius, inner_angle) = match &light.light_type
            {
                LightType::Directional { direction } => (
                    [0.0; 3],
                    [direction.x, direction.y, direction.z],
                    0.0,
                    0.0,
                    0.0,
                ),
                LightType::Point { position, radius } => {
                    (position.coords.into(), [0.0; 3], 1.0, *radius, 0.0)
                }
                LightType::Spot {
                    position,
                    direction,
                    inner_angle,
                    outer_angle,
                } => (
                    position.coords.into(),
                    [direction.x, direction.y, direction.z],
                    2.0,
                    *outer_angle,
                    *inner_angle,
                ),
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

        self.queue
            .write_buffer(&self.lighting_buffer, 0, buffer_data);
    }

    /// 更新材质缓冲区
    fn update_material_buffer(&self, material: &Material) {
        let material_uniform = MaterialUniform {
            albedo: [material.albedo.r, material.albedo.g, material.albedo.b],
            metallic: material.metallic,
            roughness: material.roughness,
            ao: material.ao,
            _padding1: [0.0; 2],
            emissive: [
                material.emissive.r,
                material.emissive.g,
                material.emissive.b,
            ],
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

        // 更新相机位置 (轨道相机 - 围绕原点旋转)
        let cos_pitch = self.camera_rotation.1.cos();
        let sin_pitch = self.camera_rotation.1.sin();
        let cos_yaw = self.camera_rotation.0.cos();
        let sin_yaw = self.camera_rotation.0.sin();

        self.camera_position = Point3::new(
            self.camera_distance * cos_pitch * cos_yaw,
            self.camera_distance * cos_pitch * sin_yaw,
            self.camera_distance * sin_pitch,
        );
        
        self.camera_dirty = true;
    }

    /// 缩放相机 (调整距离)
    pub fn zoom_camera(&mut self, factor: f32) {
        self.camera_distance = (self.camera_distance * factor).clamp(2.0, 100.0);

        // 更新相机位置
        let cos_pitch = self.camera_rotation.1.cos();
        let sin_pitch = self.camera_rotation.1.sin();
        let cos_yaw = self.camera_rotation.0.cos();
        let sin_yaw = self.camera_rotation.0.sin();

        self.camera_position = Point3::new(
            self.camera_distance * cos_pitch * cos_yaw,
            self.camera_distance * cos_pitch * sin_yaw,
            self.camera_distance * sin_pitch,
        );
        
        self.camera_dirty = true;
    }

    /// 重置相机
    pub fn reset_camera(&mut self) {
        self.camera_rotation = (0.7, 0.5); // 更好的初始角度
        self.camera_distance = 10.0;
        
        let cos_pitch = self.camera_rotation.1.cos();
        let sin_pitch = self.camera_rotation.1.sin();
        let cos_yaw = self.camera_rotation.0.cos();
        let sin_yaw = self.camera_rotation.0.sin();

        self.camera_position = Point3::new(
            self.camera_distance * cos_pitch * cos_yaw,
            self.camera_distance * cos_pitch * sin_yaw,
            self.camera_distance * sin_pitch,
        );
        
        self.camera_dirty = true;
    }

    /// 添加光源
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
        self.lights_dirty = true;
    }

    /// 设置环境光
    pub fn set_ambient_light(&mut self, color: [f32; 3], intensity: f32) {
        self.ambient_color = color;
        self.ambient_intensity = intensity;
        self.lights_dirty = true;
    }

    /// 渲染多个物体（新的批量渲染方法）
    pub fn render_multiple(
        &mut self,
        surface: &Surface,
        objects: &[(Vec<Vertex3DLit>, Vec<u16>, Material)],
        aspect_ratio: f32,
    ) -> Result<()> {
        // 只在必要时更新统一缓冲区
        if self.camera_dirty || (self.last_aspect_ratio - aspect_ratio).abs() > 0.001 {
            self.update_camera_buffer(aspect_ratio);
            self.camera_dirty = false;
            self.last_aspect_ratio = aspect_ratio;
        }
        
        if self.lights_dirty {
            self.update_lighting_buffer();
            self.lights_dirty = false;
        }

        // 为所有物体预先创建缓冲区
        let mut buffers = Vec::new();
        for (vertices, indices, material) in objects {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: BufferUsages::VERTEX,
                });

            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: BufferUsages::INDEX,
                });

            buffers.push((vertex_buffer, index_buffer, material.clone(), indices.len()));
        }

        // 获取当前帧
        let output = surface.get_current_texture().map_err(|e| {
            VizuaraError::RenderError(format!("Failed to get surface texture: {}", e))
        })?;

        // 创建深度纹理
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

        // 获取颜色视图
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 创建命令编码器
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            // 渲染所有物体
            for (vertex_buffer, index_buffer, material, index_count) in &buffers {
                // 更新材质缓冲区
                self.update_material_buffer(material);

                // 设置渲染状态并绘制
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_bind_group(1, &self.lighting_bind_group, &[]);
                render_pass.set_bind_group(2, &self.material_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..*index_count as u32, 0, 0..1);
            }
        }

        // 提交命令
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// 渲染带坐标轴的3D场景
    pub fn render_with_axes(
        &mut self,
        surface: &Surface,
        objects: &[(Vec<Vertex3DLit>, Vec<u16>, Material)],
        coordinate_system: &CoordinateSystem3D,
        aspect_ratio: f32,
    ) -> Result<()> {
        // 只在必要时更新统一缓冲区
        if self.camera_dirty || (self.last_aspect_ratio - aspect_ratio).abs() > 0.001 {
            self.update_camera_buffer(aspect_ratio);
            self.camera_dirty = false;
            self.last_aspect_ratio = aspect_ratio;
        }
        
        if self.lights_dirty {
            self.update_lighting_buffer();
            self.lights_dirty = false;
        }

        // 为所有物体预先创建缓冲区
        let mut buffers = Vec::new();
        for (vertices, indices, material) in objects {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: BufferUsages::VERTEX,
                });

            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: BufferUsages::INDEX,
                });

            buffers.push((vertex_buffer, index_buffer, material.clone(), indices.len()));
        }

        // 生成坐标轴渲染数据
        let axis_render_data = coordinate_system.generate_render_data();
        let axis_vertices = self.create_axis_vertices(&axis_render_data);
        let plane_vertices = self.create_plane_vertices(&axis_render_data);
        let text_vertices = self.create_text_vertices(&axis_render_data);
        
        let axis_vertex_buffer = if !axis_vertices.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Axis Vertex Buffer"),
                contents: bytemuck::cast_slice(&axis_vertices),
                usage: BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        let plane_vertex_buffer = if !plane_vertices.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Plane Vertex Buffer"),
                contents: bytemuck::cast_slice(&plane_vertices),
                usage: BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        let text_vertex_buffer = if !text_vertices.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Vertex Buffer"),
                contents: bytemuck::cast_slice(&text_vertices),
                usage: BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        // 获取当前帧
        let output = surface.get_current_texture().map_err(|e| {
            VizuaraError::RenderError(format!("Failed to get surface texture: {}", e))
        })?;

        // 创建深度纹理
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

        // 获取颜色视图
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 创建命令编码器
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            // 先渲染坐标面（半透明，在最后面）
            if let Some(ref plane_buffer) = plane_vertex_buffer {
                render_pass.set_pipeline(&self.plane_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, plane_buffer.slice(..));
                render_pass.draw(0..plane_vertices.len() as u32, 0..1);
            }

            // 然后渲染坐标轴线条
            if let Some(ref axis_buffer) = axis_vertex_buffer {
                render_pass.set_pipeline(&self.axis_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, axis_buffer.slice(..));
                render_pass.draw(0..axis_vertices.len() as u32, 0..1);
            }

            // 渲染所有物体
            for (vertex_buffer, index_buffer, material, index_count) in &buffers {
                // 更新材质缓冲区
                self.update_material_buffer(material);

                // 设置渲染状态并绘制
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_bind_group(1, &self.lighting_bind_group, &[]);
                render_pass.set_bind_group(2, &self.material_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..*index_count as u32, 0, 0..1);
            }
        }

        // 在提交前，绘制基于屏幕空间的 Unicode 文本覆盖（完整 Unicode 支持）
        {
            let size = output.texture.size();
            let width = size.width;
            let height = size.height;
            let _ = self.draw_overlay_texts_for_axes(
                &mut encoder,
                &view,
                aspect_ratio,
                width,
                height,
                &axis_render_data,
            );
        }

        // 第二个渲染通道：渲染3D文本（最前面）
        if let Some(ref text_buffer) = text_vertex_buffer {
            let mut text_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // 保留已有内容
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            text_render_pass.set_pipeline(&self.text_pipeline);
            text_render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            text_render_pass.set_vertex_buffer(0, text_buffer.slice(..));
            text_render_pass.draw(0..text_vertices.len() as u32, 0..1);
        }

        // 提交命令
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// 从坐标轴渲染数据创建顶点
    fn create_axis_vertices(&self, render_data: &Axis3DRenderData) -> Vec<AxisVertex> {
        let mut vertices = Vec::new();

        // 轴线（红色X，绿色Y，蓝色Z）
        for chunk in render_data.axis_lines.chunks(2) {
            if chunk.len() == 2 {
                let start = chunk[0];
                let end = chunk[1];
                
                // 根据轴的方向确定颜色
                let color = if (end.x - start.x).abs() > 0.1 {
                    [1.0, 0.0, 0.0] // X轴 - 红色
                } else if (end.y - start.y).abs() > 0.1 {
                    [0.0, 1.0, 0.0] // Y轴 - 绿色
                } else {
                    [0.0, 0.0, 1.0] // Z轴 - 蓝色
                };

                vertices.push(AxisVertex::new(start.coords.into(), color));
                vertices.push(AxisVertex::new(end.coords.into(), color));
            }
        }

        // 坐标轴盒子线条（更粗的黑色线）
        for chunk in render_data.box_lines.chunks(2) {
            if chunk.len() == 2 {
                let color = [0.2, 0.2, 0.2]; // 深灰色
                vertices.push(AxisVertex::new(chunk[0].coords.into(), color));
                vertices.push(AxisVertex::new(chunk[1].coords.into(), color));
            }
        }

        // 主刻度线（深灰色）
        for chunk in render_data.major_ticks.chunks(2) {
            if chunk.len() == 2 {
                let color = [0.4, 0.4, 0.4];
                vertices.push(AxisVertex::new(chunk[0].coords.into(), color));
                vertices.push(AxisVertex::new(chunk[1].coords.into(), color));
            }
        }

        // 次刻度线（浅灰色）
        for chunk in render_data.minor_ticks.chunks(2) {
            if chunk.len() == 2 {
                let color = [0.6, 0.6, 0.6];
                vertices.push(AxisVertex::new(chunk[0].coords.into(), color));
                vertices.push(AxisVertex::new(chunk[1].coords.into(), color));
            }
        }

        // 网格线（半透明灰色）
        for chunk in render_data.grid_lines.chunks(2) {
            if chunk.len() == 2 {
                let color = [0.7, 0.7, 0.7];
                vertices.push(AxisVertex::new(chunk[0].coords.into(), color));
                vertices.push(AxisVertex::new(chunk[1].coords.into(), color));
            }
        }

        // 次网格线（更浅的灰色）
        for chunk in render_data.minor_grid_lines.chunks(2) {
            if chunk.len() == 2 {
                let color = [0.82, 0.82, 0.82];
                vertices.push(AxisVertex::new(chunk[0].coords.into(), color));
                vertices.push(AxisVertex::new(chunk[1].coords.into(), color));
            }
        }

        // 原点标记（如果需要）
        if let Some(origin) = render_data.origin_marker {
            let size = 0.05;
            let color = [1.0, 0.0, 0.0]; // 红色
            
            // 创建一个简单的十字标记
            vertices.push(AxisVertex::new([origin.x - size, origin.y, origin.z], color));
            vertices.push(AxisVertex::new([origin.x + size, origin.y, origin.z], color));
            vertices.push(AxisVertex::new([origin.x, origin.y - size, origin.z], color));
            vertices.push(AxisVertex::new([origin.x, origin.y + size, origin.z], color));
            vertices.push(AxisVertex::new([origin.x, origin.y, origin.z - size], color));
            vertices.push(AxisVertex::new([origin.x, origin.y, origin.z + size], color));
        }

        vertices
    }

    /// 从坐标轴渲染数据创建坐标面顶点
    fn create_plane_vertices(&self, render_data: &Axis3DRenderData) -> Vec<PlaneVertex> {
        let mut vertices = Vec::new();

        // 坐标面三角形
        for (i, chunk) in render_data.plane_triangles.chunks(3).enumerate() {
            if chunk.len() == 3 {
                let color = if i < render_data.plane_colors.len() {
                    render_data.plane_colors[i]
                } else {
                    [0.8, 0.8, 0.8, 0.1] // 默认颜色
                };

                for point in chunk {
                    vertices.push(PlaneVertex::new(point.coords.into(), color));
                }
            }
        }

        vertices
    }

    /// 从坐标轴渲染数据创建3D文本顶点
    fn create_text_vertices(&self, render_data: &Axis3DRenderData) -> Vec<Text3DVertex> {
        let mut vertices = Vec::new();

        // 处理刻度标签
        for (position, text, axis_direction) in &render_data.tick_labels {
            let char_vertices = self.create_text_quad(*position, text, 0.1, [1.0, 1.0, 1.0, 1.0], *axis_direction);
            vertices.extend(char_vertices);
        }

        // 处理轴标题
        for (position, text, axis_direction) in &render_data.axis_titles {
            let char_vertices = self.create_text_quad(*position, text, 0.15, [1.0, 1.0, 0.8, 1.0], *axis_direction);
            vertices.extend(char_vertices);
        }

        vertices
    }

    /// 创建文本四边形（面向相机）
    fn create_text_quad(
        &self,
        position: Point3<f32>,
        text: &str,
        size: f32,
        color: [f32; 4],
        axis_direction: Axis3DDirection,
    ) -> Vec<Text3DVertex> {
        let mut vertices = Vec::new();
        
        // 计算文本的偏移方向（面向相机）
        let offset = match axis_direction {
            Axis3DDirection::X => Vector3::new(0.0, size * 0.5, 0.0),
            Axis3DDirection::Y => Vector3::new(size * 0.5, 0.0, 0.0),
            Axis3DDirection::Z => Vector3::new(size * 0.5, size * 0.5, 0.0),
        };

        // 为每个字符创建一个四边形
        for (i, ch) in text.chars().enumerate() {
            let char_pos = position + offset + Vector3::new(i as f32 * size * 0.65, 0.0, 0.0);
            let code = ch as u32;
            
            // 创建面向相机的四边形
            let half_size = size * 0.5;
            
            // 四个顶点（逆时针）
            let positions = [
                [char_pos.x - half_size, char_pos.y - half_size, char_pos.z],
                [char_pos.x + half_size, char_pos.y - half_size, char_pos.z],
                [char_pos.x + half_size, char_pos.y + half_size, char_pos.z],
                [char_pos.x - half_size, char_pos.y + half_size, char_pos.z],
            ];

            let tex_coords = [
                [0.0, 1.0],
                [1.0, 1.0],
                [1.0, 0.0],
                [0.0, 0.0],
            ];

            // 两个三角形组成四边形
            let indices = [0, 1, 2, 0, 2, 3];
            
            for &index in &indices {
                vertices.push(Text3DVertex::new(
                    positions[index],
                    tex_coords[index],
                    color,
                    code,
                ));
            }
        }

        vertices
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
            // 更新文本缓存尺寸
            // glyphon 的 Buffer 在 prepare 时会被重新适配
        }
    }
}
