pub mod surface;
pub mod scatter3d;
pub mod mesh;
pub mod camera;
pub mod lighting;

pub use surface::*;
pub use scatter3d::*;
pub use mesh::*;
pub use camera::*;
pub use lighting::*;

/// 3D 绘图区域
#[derive(Debug, Clone)]
pub struct Plot3DArea {
    /// X轴范围
    pub x_range: (f32, f32),
    /// Y轴范围
    pub y_range: (f32, f32),
    /// Z轴范围
    pub z_range: (f32, f32),
    /// 视图矩阵
    pub view_matrix: nalgebra::Matrix4<f32>,
    /// 投影矩阵
    pub projection_matrix: nalgebra::Matrix4<f32>,
}

impl Plot3DArea {
    pub fn new(
        x_range: (f32, f32),
        y_range: (f32, f32),
        z_range: (f32, f32)
    ) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
            view_matrix: nalgebra::Matrix4::identity(),
            projection_matrix: nalgebra::Matrix4::identity(),
        }
    }

    /// 设置透视投影
    pub fn perspective(mut self, aspect: f32, fov: f32, near: f32, far: f32) -> Self {
        self.projection_matrix = nalgebra::Matrix4::new_perspective(aspect, fov, near, far);
        self
    }

    /// 设置相机位置
    pub fn camera(mut self, eye: nalgebra::Point3<f32>, target: nalgebra::Point3<f32>, up: nalgebra::Vector3<f32>) -> Self {
        self.view_matrix = nalgebra::Matrix4::look_at_rh(&eye, &target, &up);
        self
    }
}
