//! WGPU 渲染后端
//! 
//! 使用 WGPU 提供高性能的 GPU 渲染功能

pub mod renderer;
pub mod renderer_3d;
pub mod renderer_3d_lit;
pub mod shader;
pub mod vertex;

pub use renderer::WgpuRenderer;
pub use renderer_3d::{Wgpu3DRenderer, Vertex3D};
pub use renderer_3d_lit::{Wgpu3DLitRenderer, Vertex3DLit};
pub use shader::*;
pub use vertex::*;
