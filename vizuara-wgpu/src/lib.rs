//! WGPU 渲染后端
//!
//! 使用 WGPU 提供高性能的 GPU 渲染功能

pub mod renderer;
pub mod renderer_3d;
#[cfg(feature = "lit3d")]
pub mod renderer_3d_lit;
pub mod shader;
pub mod vertex;

pub use renderer::WgpuRenderer;
pub use renderer_3d::{Vertex3D, Wgpu3DRenderer};
#[cfg(feature = "lit3d")]
pub use renderer_3d_lit::{Vertex3DLit, Wgpu3DLitRenderer};
pub use shader::*;
pub use vertex::*;
