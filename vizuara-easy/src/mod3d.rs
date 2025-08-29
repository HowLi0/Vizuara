use vizuara_core::{Color, Result};
use vizuara_3d::{CoordinateSystem3D, GridType, Surface3D};
use vizuara_window::Window3D;

/// 3D Figure 便捷封装
pub struct Figure3D {
    window: Window3D,
    coord: Option<CoordinateSystem3D>,
}

impl Figure3D {
    pub fn new() -> Self { Self { window: Window3D::new(), coord: None } }

    /// 一键默认坐标系（范围与缩放）
    pub fn with_default_coords(mut self, x:(f32,f32), y:(f32,f32), z:(f32,f32), scale:f32) -> Self {
        self.coord = Some(CoordinateSystem3D::new(x, y, z, nalgebra::Point3::new(0.0,0.0,0.0), scale)
            .grid(GridType::MajorMinor)
            .axis_titles("X","Y","Z"));
        self
    }

    /// 添加3D散点
    pub fn scatter3d(mut self, xyz: &[(f32,f32,f32)], color: Color, size: f32) -> Self {
        let scatter = vizuara_3d::Scatter3D::from_data(xyz).default_color(color).default_size(size);
        self.window = self.window.add_scatter3d(scatter);
        self
    }

    /// 添加3D表面
    pub fn surface3d(mut self, surface: Surface3D) -> Self {
        self.window = self.window.add_surface3d(surface);
        self
    }

    /// 相机快捷控制
    pub fn reset_camera(mut self) -> Self { self.window = { let mut w = self.window; w }; self }

    pub async fn show(self) -> Result<()> { self.window.run().await }
}
