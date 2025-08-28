use nalgebra::Point2;
use vizuara_core::{Color, Primitive};

/// 3D 表面图数据点
#[derive(Debug, Clone)]
pub struct SurfacePoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl SurfacePoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// 3D 表面图网格
#[derive(Debug, Clone)]
pub struct SurfaceMesh {
    /// 网格数据 (行主序)
    pub points: Vec<Vec<SurfacePoint>>,
    /// 网格宽度 (列数)
    pub width: usize,
    /// 网格高度 (行数)
    pub height: usize,
}

impl SurfaceMesh {
    /// 从函数创建表面网格
    pub fn from_function<F>(
        x_range: (f32, f32),
        y_range: (f32, f32),
        resolution: (usize, usize),
        func: F,
    ) -> Self
    where
        F: Fn(f32, f32) -> f32,
    {
        let (x_min, x_max) = x_range;
        let (y_min, y_max) = y_range;
        let (width, height) = resolution;

        let mut points = Vec::with_capacity(height);

        for i in 0..height {
            let mut row = Vec::with_capacity(width);
            let y = y_min + (y_max - y_min) * i as f32 / (height - 1) as f32;

            for j in 0..width {
                let x = x_min + (x_max - x_min) * j as f32 / (width - 1) as f32;
                let z = func(x, y);
                row.push(SurfacePoint::new(x, y, z));
            }
            points.push(row);
        }

        Self {
            points,
            width,
            height,
        }
    }

    /// 从2D数组创建表面网格
    pub fn from_grid(x_coords: &[f32], y_coords: &[f32], z_values: &[Vec<f32>]) -> Self {
        let height = y_coords.len();
        let width = x_coords.len();
        let mut points = Vec::with_capacity(height);

        for (i, &y) in y_coords.iter().enumerate() {
            let mut row = Vec::with_capacity(width);
            for (j, &x) in x_coords.iter().enumerate() {
                let z = z_values[i][j];
                row.push(SurfacePoint::new(x, y, z));
            }
            points.push(row);
        }

        Self {
            points,
            width,
            height,
        }
    }

    /// 获取指定位置的点
    pub fn get_point(&self, row: usize, col: usize) -> Option<&SurfacePoint> {
        self.points.get(row)?.get(col)
    }

    /// 获取指定坐标的点
    pub fn point_at(&self, x: usize, y: usize) -> Option<nalgebra::Point3<f32>> {
        self.points
            .get(y)?
            .get(x)
            .map(|p| nalgebra::Point3::new(p.x, p.y, p.z))
    }

    /// 获取数据边界
    pub fn bounds(&self) -> ((f32, f32), (f32, f32), (f32, f32)) {
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for row in &self.points {
            for point in row {
                min_x = min_x.min(point.x);
                max_x = max_x.max(point.x);
                min_y = min_y.min(point.y);
                max_y = max_y.max(point.y);
                min_z = min_z.min(point.z);
                max_z = max_z.max(point.z);
            }
        }

        ((min_x, max_x), (min_y, max_y), (min_z, max_z))
    }
}

/// 3D 表面图样式
#[derive(Debug, Clone)]
pub struct SurfaceStyle {
    /// 是否显示线框
    pub wireframe: bool,
    /// 线框颜色
    pub wireframe_color: Color,
    /// 线框宽度
    pub wireframe_width: f32,
    /// 是否填充表面
    pub fill_surface: bool,
    /// 表面颜色映射函数
    pub color_map: fn(f32) -> Color,
    /// 是否启用光照
    pub enable_lighting: bool,
}

impl Default for SurfaceStyle {
    fn default() -> Self {
        Self {
            wireframe: true,
            wireframe_color: Color::rgb(0.3, 0.3, 0.3),
            wireframe_width: 1.0,
            fill_surface: false,
            color_map: |_| Color::rgb(0.5, 0.7, 1.0),
            enable_lighting: false,
        }
    }
}

/// 3D 表面图
#[derive(Debug, Clone)]
pub struct Surface3D {
    mesh: SurfaceMesh,
    style: SurfaceStyle,
}

impl Surface3D {
    /// 创建新的表面图
    pub fn new(mesh: SurfaceMesh) -> Self {
        Self {
            mesh,
            style: SurfaceStyle::default(),
        }
    }

    /// 从函数创建表面图
    pub fn from_function<F>(
        x_range: (f32, f32),
        y_range: (f32, f32),
        resolution: (usize, usize),
        func: F,
    ) -> Self
    where
        F: Fn(f32, f32) -> f32,
    {
        let mesh = SurfaceMesh::from_function(x_range, y_range, resolution, func);
        Self::new(mesh)
    }

    /// 设置样式
    pub fn style(mut self, style: SurfaceStyle) -> Self {
        self.style = style;
        self
    }

    /// 启用线框模式
    pub fn wireframe(mut self, enable: bool) -> Self {
        self.style.wireframe = enable;
        self
    }

    /// 设置线框颜色
    pub fn wireframe_color(mut self, color: Color) -> Self {
        self.style.wireframe_color = color;
        self
    }

    /// 获取网格数据
    pub fn mesh(&self) -> &SurfaceMesh {
        &self.mesh
    }

    /// 生成渲染图元 (简化的2D投影)
    pub fn generate_primitives(&self, _plot_area: &crate::Plot3DArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if !self.style.wireframe {
            return primitives;
        }

        // 简化的投影：忽略Z坐标，直接映射X,Y到屏幕坐标
        let ((x_min, x_max), (y_min, y_max), _) = self.mesh.bounds();
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        // 假设屏幕区域是 800x600
        let screen_width = 600.0;
        let screen_height = 400.0;
        let offset_x = 100.0;
        let offset_y = 100.0;

        // 绘制水平线
        for i in 0..self.mesh.height {
            for j in 0..(self.mesh.width - 1) {
                if let (Some(p1), Some(p2)) =
                    (self.mesh.get_point(i, j), self.mesh.get_point(i, j + 1))
                {
                    let x1 = offset_x + (p1.x - x_min) / x_range * screen_width;
                    let y1 = offset_y + screen_height - (p1.y - y_min) / y_range * screen_height;
                    let x2 = offset_x + (p2.x - x_min) / x_range * screen_width;
                    let y2 = offset_y + screen_height - (p2.y - y_min) / y_range * screen_height;

                    primitives.push(Primitive::Line {
                        start: Point2::new(x1, y1),
                        end: Point2::new(x2, y2),
                    });
                }
            }
        }

        // 绘制垂直线
        for j in 0..self.mesh.width {
            for i in 0..(self.mesh.height - 1) {
                if let (Some(p1), Some(p2)) =
                    (self.mesh.get_point(i, j), self.mesh.get_point(i + 1, j))
                {
                    let x1 = offset_x + (p1.x - x_min) / x_range * screen_width;
                    let y1 = offset_y + screen_height - (p1.y - y_min) / y_range * screen_height;
                    let x2 = offset_x + (p2.x - x_min) / x_range * screen_width;
                    let y2 = offset_y + screen_height - (p2.y - y_min) / y_range * screen_height;

                    primitives.push(Primitive::Line {
                        start: Point2::new(x1, y1),
                        end: Point2::new(x2, y2),
                    });
                }
            }
        }

        primitives
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_point_creation() {
        let point = SurfacePoint::new(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn test_surface_mesh_from_function() {
        let mesh =
            SurfaceMesh::from_function((-1.0, 1.0), (-1.0, 1.0), (3, 3), |x, y| x * x + y * y);

        assert_eq!(mesh.width, 3);
        assert_eq!(mesh.height, 3);
        assert_eq!(mesh.points.len(), 3);
        assert_eq!(mesh.points[0].len(), 3);
    }

    #[test]
    fn test_surface_mesh_bounds() {
        let mesh = SurfaceMesh::from_function((-2.0, 2.0), (-1.0, 1.0), (3, 3), |x, y| x + y);

        let bounds = mesh.bounds();
        assert_eq!(bounds.0, (-2.0, 2.0)); // X bounds
        assert_eq!(bounds.1, (-1.0, 1.0)); // Y bounds
    }

    #[test]
    fn test_surface3d_creation() {
        let mesh = SurfaceMesh::from_function((0.0, 1.0), (0.0, 1.0), (2, 2), |x, y| x + y);

        let surface = Surface3D::new(mesh);
        assert_eq!(surface.mesh().width, 2);
        assert_eq!(surface.mesh().height, 2);
    }

    #[test]
    fn test_surface3d_from_function() {
        let surface = Surface3D::from_function((-1.0, 1.0), (-1.0, 1.0), (5, 5), |x, y| {
            (x * x + y * y).sin()
        });

        assert_eq!(surface.mesh().width, 5);
        assert_eq!(surface.mesh().height, 5);
    }

    #[test]
    fn test_surface_style() {
        let mesh = SurfaceMesh::from_function((0.0, 1.0), (0.0, 1.0), (2, 2), |_, _| 0.0);

        let surface = Surface3D::new(mesh)
            .wireframe(true)
            .wireframe_color(Color::rgb(1.0, 0.0, 0.0));

        assert!(surface.style.wireframe);
        assert_eq!(surface.style.wireframe_color, Color::rgb(1.0, 0.0, 0.0));
    }
}
