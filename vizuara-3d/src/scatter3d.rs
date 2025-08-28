use crate::BoundingBox3D;
use nalgebra::Point3;
use vizuara_core::{Color, Primitive};

/// 3D 散点图数据点
#[derive(Debug, Clone)]
pub struct Point3D {
    pub position: Point3<f32>,
    pub color: Color,
    pub size: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Point3::new(x, y, z),
            color: Color::rgb(0.5, 0.5, 1.0),
            size: 5.0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

/// 3D 散点图
#[derive(Debug, Clone)]
pub struct Scatter3D {
    points: Vec<Point3D>,
    default_color: Color,
    default_size: f32,
}

impl Scatter3D {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            default_color: Color::rgb(0.3, 0.6, 1.0),
            default_size: 6.0,
        }
    }

    /// 从 3D 坐标数据创建散点图
    pub fn from_data(data: &[(f32, f32, f32)]) -> Self {
        let mut scatter = Self::new();
        for &(x, y, z) in data {
            scatter.points.push(
                Point3D::new(x, y, z)
                    .color(scatter.default_color)
                    .size(scatter.default_size),
            );
        }
        scatter
    }

    /// 添加单个点
    pub fn add_point(mut self, point: Point3D) -> Self {
        self.points.push(point);
        self
    }

    /// 设置默认颜色
    pub fn default_color(mut self, color: Color) -> Self {
        self.default_color = color;
        self
    }

    /// 设置默认大小
    pub fn default_size(mut self, size: f32) -> Self {
        self.default_size = size;
        self
    }

    /// 获取点的数量
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// 获取数据边界
    pub fn bounds(&self) -> Option<BoundingBox3D> {
        if self.points.is_empty() {
            return None;
        }

        let first = &self.points[0].position;
        let mut min_x = first.x;
        let mut max_x = first.x;
        let mut min_y = first.y;
        let mut max_y = first.y;
        let mut min_z = first.z;
        let mut max_z = first.z;

        for point in &self.points {
            let pos = &point.position;
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x);
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y);
            min_z = min_z.min(pos.z);
            max_z = max_z.max(pos.z);
        }

        Some(((min_x, max_x), (min_y, max_y), (min_z, max_z)))
    }

    /// 获取指定索引的点
    pub fn point_at(&self, index: usize) -> Option<Point3<f32>> {
        self.points.get(index).map(|p| p.position)
    }

    /// 获取指定索引的颜色
    pub fn color_at(&self, index: usize) -> Option<Color> {
        self.points.get(index).map(|p| p.color)
    }

    /// 生成渲染图元 (将3D点投影到2D进行渲染)
    pub fn generate_primitives(&self, plot_area: &crate::Plot3DArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        // 创建模型-视图-投影矩阵
        let mvp = plot_area.projection_matrix * plot_area.view_matrix;

        for point in &self.points {
            // 将3D点转换为齐次坐标
            let world_pos =
                nalgebra::Vector4::new(point.position.x, point.position.y, point.position.z, 1.0);

            // 应用变换矩阵
            let clip_pos = mvp * world_pos;

            // 透视分割
            if clip_pos.w != 0.0 {
                let ndc_x = clip_pos.x / clip_pos.w;
                let ndc_y = clip_pos.y / clip_pos.w;
                let ndc_z = clip_pos.z / clip_pos.w;

                // 检查是否在视锥体内
                if (-1.0..=1.0).contains(&ndc_x)
                    && (-1.0..=1.0).contains(&ndc_y)
                    && (0.0..=1.0).contains(&ndc_z)
                {
                    // 转换到屏幕坐标 (假设800x600窗口)
                    let screen_x = (ndc_x + 1.0) * 400.0;
                    let screen_y = (1.0 - ndc_y) * 300.0;

                    // 根据深度调整点的大小和颜色
                    let depth_factor = 1.0 - ndc_z; // 越近越大
                    let adjusted_size = point.size * depth_factor;
                    let _adjusted_color = Color::new(
                        point.color.r * depth_factor,
                        point.color.g * depth_factor,
                        point.color.b * depth_factor,
                        point.color.a,
                    );

                    // 创建圆形图元
                    primitives.push(Primitive::Circle {
                        center: nalgebra::Point2::new(screen_x, screen_y),
                        radius: adjusted_size,
                    });
                }
            }
        }

        primitives
    }
}

impl Default for Scatter3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scatter3d_creation() {
        let scatter = Scatter3D::new();
        assert_eq!(scatter.point_count(), 0);
    }

    #[test]
    fn test_scatter3d_from_data() {
        let data = [(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (2.0, 2.0, 2.0)];
        let scatter = Scatter3D::from_data(&data);
        assert_eq!(scatter.point_count(), 3);
    }

    #[test]
    fn test_scatter3d_bounds() {
        let data = [(1.0, 2.0, 3.0), (4.0, 5.0, 6.0), (0.0, 1.0, 2.0)];
        let scatter = Scatter3D::from_data(&data);

        let bounds = scatter.bounds().unwrap();
        assert_eq!(bounds.0, (0.0, 4.0)); // X bounds
        assert_eq!(bounds.1, (1.0, 5.0)); // Y bounds
        assert_eq!(bounds.2, (2.0, 6.0)); // Z bounds
    }

    #[test]
    fn test_point3d_creation() {
        let point = Point3D::new(1.0, 2.0, 3.0)
            .color(Color::rgb(1.0, 0.0, 0.0))
            .size(10.0);

        assert_eq!(point.position.x, 1.0);
        assert_eq!(point.position.y, 2.0);
        assert_eq!(point.position.z, 3.0);
        assert_eq!(point.size, 10.0);
    }

    #[test]
    fn test_scatter3d_add_point() {
        let scatter = Scatter3D::new().add_point(Point3D::new(1.0, 2.0, 3.0));

        assert_eq!(scatter.point_count(), 1);
    }

    #[test]
    fn test_empty_scatter3d_bounds() {
        let scatter = Scatter3D::new();
        assert!(scatter.bounds().is_none());
    }
}
