//! Easy, Matlab-like API facade for Vizuara
//! 提供类似 Matlab/Matplotlib figure 的简单 2D/3D 绘图门面 API

pub mod mod2d;
pub mod mod3d;

pub mod prelude {
    pub use crate::mod2d::{Figure2D, Colors, testdata};
    pub use crate::mod3d::Figure3D;
    pub use crate::{figure, figure3d, figure_std, figure_large};
    pub use vizuara_core::Color;
}

/// 创建2D图形
pub fn figure(width: f32, height: f32) -> mod2d::Figure2D { 
    mod2d::Figure2D::new(width, height) 
}

/// 创建3D图形
pub fn figure3d() -> mod3d::Figure3D { 
    mod3d::Figure3D::new() 
}

/// 快速创建标准尺寸的2D图形
pub fn figure_std() -> mod2d::Figure2D {
    mod2d::Figure2D::new(800.0, 600.0)
}

/// 快速创建大尺寸的2D图形
pub fn figure_large() -> mod2d::Figure2D {
    mod2d::Figure2D::new(1200.0, 800.0)
}
